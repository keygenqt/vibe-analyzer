//! Indexer coordinator for OpenSearch operations.
//! Manages bulk indexing for meta, files, and analysis indices.

use crate::opensearch::client::OpenSearchClient;
use crate::opensearch::data::source_file::SourceFile;
use crate::opensearch::data::source_file_analysis::SourceFileAnalysis;
use crate::opensearch::data::source_meta::SourceMeta;
use crate::opensearch::index::index_files::FilesIndex;
use crate::opensearch::index::index_files_analysis::FilesAnalysisIndex;
use crate::opensearch::index::index_meta::MetaIndex;
use crate::utils::hash::hash_from_path;
use anyhow::Result;
use std::path::Path;

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
        project_root: &Path,
        files: &[SourceFile],
        analyses: &[SourceFileAnalysis],
        force: bool,
    ) -> Result<IndexFilesReport> {
        let hash = hash_from_path(project_root);
        let files_index = FilesIndex::new(self.client.clone(), &hash);
        let analysis_index = FilesAnalysisIndex::new(self.client.clone(), &hash);

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
}
