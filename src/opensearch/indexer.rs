//! Indexer coordinator for OpenSearch operations.
//! Manages bulk indexing for meta, files, and analysis indices.

use crate::configs::app::{AppConfig, IndexType};
use crate::opensearch::client::OpenSearchClient;
use crate::opensearch::data::requests::source_file::SourceFile;
use crate::opensearch::data::requests::source_file_analysis::SourceFileAnalysis;
use crate::opensearch::data::requests::source_meta::SourceMeta;
use crate::opensearch::index::index_files::FilesIndex;
use crate::opensearch::index::index_files_analysis::FilesAnalysisIndex;
use crate::opensearch::index::index_meta::MetaIndex;
use crate::opensearch::search::SearchClient;
use crate::utils;
use anyhow::Result;
use serde_json::json;
use std::collections::HashSet;
use std::path::PathBuf;

/// Report for indexing operations.
#[derive(Debug, Default)]
pub struct IndexReport {
    pub meta: IndexMetaReport,
    pub files: IndexFilesReport,
}

/// Report for file indexing operations.
#[derive(Debug, Default)]
pub struct IndexFilesReport {
    pub files_indexed: usize,
    pub analysis_indexed: usize,
}

/// Report for metadata indexing operations.
#[derive(Debug, Default)]
pub struct IndexMetaReport {
    pub indexed: usize,
}

/// Coordinates indexing across meta, files, and analysis indices.
pub struct Indexer {
    client: OpenSearchClient,
}

impl Indexer {
    /// Creates a new Indexer.
    pub fn new(client: OpenSearchClient) -> Self {
        Self { client }
    }

    /// Indexes file content and analysis data for a project.
    pub async fn index_files(
        &self,
        root: &PathBuf,
        files: &[SourceFile],
        analyses: &[SourceFileAnalysis],
        force: bool,
    ) -> Result<IndexFilesReport> {
        let files_index = FilesIndex::new(self.client.clone(), &root);
        let analysis_index = FilesAnalysisIndex::new(self.client.clone(), &root);

        // Force reindex: delete and recreate all indices
        if force {
            if files_index.exists().await? {
                files_index.delete().await?;
            }
            if analysis_index.exists().await? {
                analysis_index.delete().await?;
            }
        }

        // Ensure indices exist before indexing
        if !files_index.exists().await? {
            files_index.create().await?;
        }
        if !analysis_index.exists().await? {
            analysis_index.create().await?;
        }

        // Start bulk indexing
        let mut report = IndexFilesReport::default();

        if !files.is_empty() {
            let docs: Vec<&SourceFile> = files.iter().collect();
            files_index.bulk_index(&docs).await?;
            report.files_indexed = files.len();
        }

        if !analyses.is_empty() {
            let docs: Vec<&SourceFileAnalysis> = analyses.iter().collect();
            analysis_index.bulk_index(&docs).await?;
            report.analysis_indexed = analyses.len();
        }

        Ok(report)
    }

    /// Indexes project metadata.
    pub async fn index_meta(&self, meta: &[SourceMeta], force: bool) -> Result<IndexMetaReport> {
        let meta_index = MetaIndex::new(self.client.clone());

        // Force reindex: delete and recreate meta index
        if force {
            if meta_index.exists().await? {
                meta_index.delete().await?;
            }
        }

        // Ensure index exists before indexing
        if !meta_index.exists().await? {
            meta_index.create().await?;
        }

        // Start bulk indexing
        let mut report = IndexMetaReport::default();

        if !meta.is_empty() {
            let docs: Vec<&SourceMeta> = meta.iter().collect();
            meta_index.bulk_index(&docs).await?;
            report.indexed = meta.len();
        }

        Ok(report)
    }

    /// Removes all orphaned data from OpenSearch.
    pub async fn cleanup(&self, config: &AppConfig) -> Result<()> {
        let search_client = SearchClient::new(self.client.clone());

        // Expected indices from current config
        let expected_indices: HashSet<String> =
            config.get_indexes(IndexType::All).into_iter().collect();

        // All indices in OpenSearch
        let all_indices = self.client.list_indices().await?;

        // Delete indices for removed sources
        for index in &all_indices {
            if index.starts_with("vibe-analyzer-") && !expected_indices.contains(index) {
                let _ = self.client.delete_index(index).await;
            }
        }

        // Delete orphaned files for each active source
        for source in config.get_sources() {
            // Files on disk
            let Ok(fs_files) = utils::path::source_files(&source, &None) else {
                continue;
            };
            let fs_paths: HashSet<String> = fs_files
                .into_iter()
                .map(|f| f.path.to_string_lossy().to_string())
                .collect();

            // Files in index
            let files_index = AppConfig::files_index(&source);
            let Ok(indexed_hashes) = search_client.get_hashes(&files_index).await else {
                continue;
            };

            let indexed_paths: HashSet<String> =
                indexed_hashes.into_iter().map(|h| h.path).collect();

            // Orphans = in index but not on disk
            let root_str = source.to_string_lossy().to_string();
            let orphans: Vec<&str> = indexed_paths
                .difference(&fs_paths)
                .map(|s| {
                    s.strip_prefix(&root_str)
                        .unwrap_or(s)
                        .trim_start_matches('/')
                })
                .collect();

            if !orphans.is_empty() {
                let analysis_index = AppConfig::files_analysis_index(&source);
                let _ = self
                    .client
                    .delete_by_full_path(&files_index, &root_str, &orphans)
                    .await;
                let _ = self
                    .client
                    .delete_by_full_path(&analysis_index, &root_str, &orphans)
                    .await;
            }
        }

        // Delete meta docs for removed sources
        let meta_index = AppConfig::meta_index();
        if all_indices.contains(&meta_index) {
            let active_paths: Vec<String> = config
                .get_sources()
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect();

            if !active_paths.is_empty() {
                let query = json!({
                    "query": {
                        "bool": {
                            "must_not": {
                                "terms": { "path.keyword": active_paths }
                            }
                        }
                    }
                });
                let _ = self.client.delete_by_query(&meta_index, &query).await;
            }
        }

        Ok(())
    }
}
