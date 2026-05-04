//! OpenSearch HTTP client.
//! Provides base HTTP methods and healthcheck for OpenSearch cluster.

use anyhow::Context;
use anyhow::Result;
use blake3::Hasher;
use reqwest::Client;
use serde_json::Value;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use crate::configs::app::AppConfig;
use crate::opensearch::indexes::index_client::OpenSearchIndexClient;
use crate::opensearch::tools::tool_client::OpenSearchToolClient;
use crate::utils::app;

pub const PROJECT_META_SUFFIX: &str = "project-meta";
pub const FILES_META_SUFFIX: &str = "files-analysis";
pub const FILES_SUFFIX: &str = "files";

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
    pub fn client_index(&self, name: &str) -> OpenSearchIndexClient {
        OpenSearchIndexClient::new(self, name.to_string())
    }

    /// Create tools client for the meta index.
    pub async fn client_tool_projects_meta(&self) -> Result<OpenSearchToolClient> {
        let index = self.client_index(&self.projects_meta_index());
        if !index.index_exists().await? {
            anyhow::bail!("Index '{}' does not exist", index.name);
        }
        Ok(OpenSearchToolClient::new(self, vec![index.name]))
    }

    /// Create tools client for all files_meta indices.
    pub async fn client_tool_files_meta(
        &self,
        target: Option<String>,
    ) -> Result<OpenSearchToolClient> {
        let sources = if let Some(t) = target {
            vec![self.config.get_source(&t)?]
        } else {
            self.config.get_sources()
        };

        let mut names = vec![];
        for source in &sources {
            let index = self.client_index(&self.files_meta_index(source));
            if index.index_exists().await? {
                names.push(index.name);
            }
        }
        if names.is_empty() {
            anyhow::bail!("No files_meta indices found. Run indexing first.");
        }
        Ok(OpenSearchToolClient::new(self, names))
    }

    /// Create tools client for all files indices.
    pub async fn client_tool_files(&self, target: Option<String>) -> Result<OpenSearchToolClient> {
        let sources = if let Some(t) = target {
            vec![self.config.get_source(&t)?]
        } else {
            self.config.get_sources()
        };
        let mut names = vec![];
        for source in &sources {
            let index = self.client_index(&self.files_index(source));
            if index.index_exists().await? {
                names.push(index.name);
            }
        }
        if names.is_empty() {
            anyhow::bail!("No files indices found. Run indexing first.");
        }
        Ok(OpenSearchToolClient::new(self, names))
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
        format!("{}-{}", app::app_name(), PROJECT_META_SUFFIX)
    }

    /// Get files analysis index name for a source.
    pub fn files_meta_index(&self, root: &PathBuf) -> String {
        format!(
            "{}-{}-{}",
            app::app_name(),
            FILES_META_SUFFIX,
            self.hash_from_path(root)
        )
    }

    /// Get files index name for a source.
    pub fn files_index(&self, root: &PathBuf) -> String {
        format!(
            "{}-{}-{}",
            app::app_name(),
            FILES_SUFFIX,
            self.hash_from_path(root)
        )
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

    /// Computes BLAKE3 hash from a string path for project identification.
    /// Returns first 8 characters of the hash.
    fn hash_from_path(&self, path: &Path) -> String {
        let mut hasher = Hasher::new();
        hasher.update(path.to_string_lossy().as_bytes());
        hasher.finalize().to_hex()[..8].to_string()
    }
}
