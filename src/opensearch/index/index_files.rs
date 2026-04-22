//! Files index management for raw file content storage.
//! Handles creation and management of vibe_files_{hash} indices.

use std::path::PathBuf;

use crate::configs::app::AppConfig;
use crate::opensearch::client::OpenSearchClient;
use crate::opensearch::data::requests::opensearch_file_content_request::OpensearchFileContentRequest;
use anyhow::Result;
use serde_json::json;

/// Index handler for raw file content.
pub struct FilesIndex {
    client: OpenSearchClient,
    name: String,
}

impl FilesIndex {
    /// Creates new FilesIndex instance.
    pub fn new(client: OpenSearchClient, source: &PathBuf) -> Self {
        Self {
            client,
            name: AppConfig::files_index(source),
        }
    }

    /// Creates index with proper mapping.
    pub async fn create(&self) -> Result<()> {
        let mapping = json!({
            "mappings": {
                "dynamic": "strict",
                "properties": {
                    "root": { "type": "keyword" },
                    "path": { "type": "keyword" },
                    "hash": { "type": "keyword" },
                    "content": { "type": "text", "index": false, "store": true }
                }
            }
        });

        self.client.create_index(&self.name, mapping).await
    }

    /// Checks if index exists.
    pub async fn exists(&self) -> Result<bool> {
        self.client.index_exists(&self.name).await
    }

    /// Deletes index.
    pub async fn delete(&self) -> Result<()> {
        self.client.delete_index(&self.name).await
    }

    /// Bulk indexes multiple files.
    pub async fn bulk_index(&self, docs: &[&OpensearchFileContentRequest]) -> Result<()> {
        self.client.bulk_index(&self.name, docs).await
    }
}
