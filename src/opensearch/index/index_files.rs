//! Files index management for raw file content storage.
//! Handles creation and management of vibe_files_{hash} indices.

use crate::{
    opensearch::{client::OpenSearchClient, data::source_file::SourceFile},
    utils::app,
};
use anyhow::Result;
use serde_json::json;

/// Index handler for raw file content.
pub struct FilesIndex {
    client: OpenSearchClient,
    name: String,
}

impl FilesIndex {
    /// Creates new FilesIndex instance.
    pub fn new(client: OpenSearchClient, project_hash: &str) -> Self {
        Self {
            client,
            name: format!("{}-files-{}", app::app_name(), project_hash),
        }
    }

    /// Creates index with proper mapping.
    pub async fn create(&self) -> Result<()> {
        let mapping = json!({
            "mappings": {
                "properties": {
                    "Root": { "type": "keyword" },
                    "Path": { "type": "keyword" },
                    "Content": { "type": "text", "index": false, "store": true }
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
    pub async fn bulk_index(&self, docs: &[&SourceFile]) -> Result<()> {
        self.client.bulk_index(&self.name, docs).await
    }
}
