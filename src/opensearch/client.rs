//! OpenSearch HTTP client.
//! Provides base HTTP methods and healthcheck for OpenSearch cluster.

use anyhow::Context;
use anyhow::Result;
use blake3::Hasher;
use reqwest::Client;
use serde_json::Value;
use serde_json::json;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use crate::configs::app::AppConfig;
use crate::models::file_model::FileModel;
use crate::models::file_models::FileModels;
use crate::models::project::ProjectModel;
use crate::opensearch::indexes::indexes_client::OpenSearchIndexesClient;
use crate::utils::app;

#[derive(Clone)]
pub struct OpenSearchClient {
    config: AppConfig,
    client: Client,
    base_url: String,
}

impl OpenSearchClient {
    /// Create new client with 30s timeout.
    pub fn new(config: &AppConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            config: config.clone(),
            client,
            base_url: config.opensearch.host.trim_end_matches('/').to_string(),
        })
    }

    /// Create indexes client for the given index name.
    pub fn indexes(&self, name: &str) -> OpenSearchIndexesClient {
        OpenSearchIndexesClient::new(self, name.to_string())
    }

    /// Check if OpenSearch cluster is reachable.
    pub async fn healthcheck(&self) -> Result<bool> {
        match self
            .client
            .get(&self.base_url)
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// Build a HEAD request to the given path.
    pub fn head(&self, path: &str) -> reqwest::RequestBuilder {
        self.client.head(&format!("{}{}", self.base_url, path))
    }

    /// Build a GET request to the given path.
    pub fn get(&self, path: &str) -> reqwest::RequestBuilder {
        self.client.get(&format!("{}{}", self.base_url, path))
    }

    /// Build a POST request to the given path.
    pub fn post(&self, path: &str) -> reqwest::RequestBuilder {
        self.client.post(&format!("{}{}", self.base_url, path))
    }

    /// Build a PUT request to the given path.
    pub fn put(&self, path: &str) -> reqwest::RequestBuilder {
        self.client.put(&format!("{}{}", self.base_url, path))
    }

    /// Build a DELETE request to the given path.
    pub fn delete(&self, path: &str) -> reqwest::RequestBuilder {
        self.client.delete(&format!("{}{}", self.base_url, path))
    }

    /// Get meta index name.
    pub fn projects_meta_index(&self) -> String {
        format!("{}-meta", app::app_name())
    }

    /// Get files analysis index name for a source.
    pub fn files_meta_index(&self, root: &PathBuf) -> String {
        format!(
            "{}-files-analysis-{}",
            app::app_name(),
            self.hash_from_path(root)
        )
    }

    /// Get files index name for a source.
    pub fn files_index(&self, root: &PathBuf) -> String {
        format!("{}-files-{}", app::app_name(), self.hash_from_path(root))
    }

    /// List all index names.
    pub async fn indexes_list(&self) -> Result<Vec<String>> {
        let response = self
            .get("/_cat/indices?format=json")
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to list indices: {}", e))?;
        let indices: Vec<Value> = response.json().await?;
        Ok(indices
            .into_iter()
            .filter_map(|v| v["index"].as_str().map(String::from))
            .collect())
    }

    /// Clean up orphaned documents across all indexes.
    pub async fn indexes_cleanup(&self) -> Result<()> {
        let sources = self.config.get_sources();
        let active_roots: Vec<String> = sources
            .iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect();

        // Clean orphaned projects from meta index
        self.indexes(&self.projects_meta_index())
            .cleanup_orphaned_projects(&active_roots)
            .await?;

        // Clean orphaned files for each source
        for source in &sources {
            let active_files = vibe_fs::scan(source, true)?;

            self.indexes(&self.files_meta_index(source))
                .cleanup_orphaned_files(&active_files)
                .await?;

            self.indexes(&self.files_index(source))
                .cleanup_orphaned_files(&active_files)
                .await?;
        }

        // Delete indexes for removed sources
        let all_indices = self.indexes_list().await?;
        let expected: Vec<String> = sources
            .iter()
            .flat_map(|s| [self.files_index(s), self.files_meta_index(s)])
            .chain(std::iter::once(self.projects_meta_index()))
            .collect();

        for index in all_indices {
            if index.starts_with(&format!("{}-", app::app_name())) && !expected.contains(&index) {
                self.indexes(&index).index_delete().await?;
            }
        }

        Ok(())
    }

    /// Create all required indexes if they don't exist.
    pub async fn indexes_create(&self) -> Result<()> {
        let sources = self.config.get_sources();
        // Create project meta index
        let meta = self.indexes(&self.projects_meta_index());
        if !meta.index_exists().await? {
            meta.create_project_meta().await?;
        }
        // Create file indexes for each source
        for source in &sources {
            let files_meta = self.indexes(&self.files_meta_index(source));
            if !files_meta.index_exists().await? {
                files_meta.create_file_meta().await?;
            }
            let files = self.indexes(&self.files_index(source));
            if !files.index_exists().await? {
                files.create_file_raw().await?;
            }
        }
        Ok(())
    }

    /// Index a project: meta, file analysis, and file content.
    pub async fn index_update(&self, project: &ProjectModel) -> Result<()> {
        let root = PathBuf::from(&project.path);

        // Index project meta
        let meta = self.indexes(&self.projects_meta_index());
        let meta_json = serde_json::to_value(project.format_index())?;
        meta.bulk(&[(project.path.clone(), meta_json)]).await?;

        // Index file analysis (AST + enrichment)
        let files_meta = self.indexes(&self.files_meta_index(&root));
        let meta_docs: Vec<(String, serde_json::Value)> = project
            .files
            .iter()
            .map(|f| match f {
                FileModels::Language(m) => m.format_index(),
                FileModels::Text(m) => m.format_index(),
                FileModels::Binary(m) => m.format_index(),
            })
            .map(|f| {
                let path = match &f {
                    FileModels::Language(m) => m.path().to_string(),
                    FileModels::Text(m) => m.path().to_string(),
                    FileModels::Binary(m) => m.path().to_string(),
                };
                let json = serde_json::to_value(&f)?;
                Ok((path, json))
            })
            .collect::<Result<Vec<_>>>()?;
        files_meta.bulk(&meta_docs).await?;

        // Index file raw content
        let files_raw = self.indexes(&self.files_index(&root));
        let raw_docs: Vec<(String, serde_json::Value)> = project
            .files
            .iter()
            .filter_map(|f| {
                let path = match f {
                    FileModels::Language(m) => m.path(),
                    FileModels::Text(m) => m.path(),
                    FileModels::Binary(_) => return None,
                };
                let content = match f {
                    FileModels::Language(m) => m.content.clone(),
                    FileModels::Text(m) => m.content.clone(),
                    FileModels::Binary(_) => None,
                };
                content.map(|c| (path.to_string(), json!({"path": path, "content": c})))
            })
            .collect();
        files_raw.bulk(&raw_docs).await?;

        Ok(())
    }

    /// Delete all indexes for this application.
    pub async fn indexes_delete(&self) -> Result<()> {
        let all_indices = self.indexes_list().await?;
        for index in all_indices {
            if index.starts_with(&format!("{}-", app::app_name())) {
                self.indexes(&index).index_delete().await?;
            }
        }
        Ok(())
    }

    /// Computes BLAKE3 hash from a string path for project identification.
    /// Returns first 8 characters of the hash.
    fn hash_from_path(&self, path: &Path) -> String {
        let mut hasher = Hasher::new();
        hasher.update(path.to_string_lossy().as_bytes());
        hasher.finalize().to_hex()[..8].to_string()
    }
}
