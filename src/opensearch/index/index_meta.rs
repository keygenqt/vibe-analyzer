//! Metadata index management for project-level information.
//! Handles creation and management of the vibe_meta index.

use crate::opensearch::client::OpenSearchClient;
use crate::opensearch::data::source_meta::SourceMeta;
use crate::utils::app;
use anyhow::Result;
use serde_json::json;

/// Index handler for project metadata.
pub struct MetaIndex {
    client: OpenSearchClient,
    name: String,
}

impl MetaIndex {
    /// Creates a new MetaIndex instance.
    pub fn new(client: OpenSearchClient) -> Self {
        Self {
            client,
            name: format!("{}-meta", app::app_name()),
        }
    }

    /// Creates the index with proper mapping.
    pub async fn create(&self) -> Result<()> {
        let mapping = json!({
            "mappings": {
                "properties": {
                    "Root": { "type": "keyword" },
                    "Summary": { "type": "text" },
                    "Readme": {
                        "properties": {
                            "Path": { "type": "keyword" },
                            "Content": { "type": "text", "index": false, "store": true },
                            "SizeBytes": { "type": "long" }
                        }
                    },
                    "License": {
                        "properties": {
                            "SpdxId": { "type": "keyword" },
                            "Name": { "type": "text" },
                            "Confidence": { "type": "float" },
                            "Path": { "type": "keyword" }
                        }
                    },
                    "Stats": {
                        "properties": {
                            "TotalFiles": { "type": "integer" },
                            "TextFiles": { "type": "integer" },
                            "BinaryFiles": { "type": "integer" },
                            "TotalFunctions": { "type": "integer" },
                            "TotalClasses": { "type": "integer" },
                            "TotalStructs": { "type": "integer" },
                            "TotalEnums": { "type": "integer" },
                            "TotalInterfaces": { "type": "integer" },
                            "TotalImports": { "type": "integer" },
                            "TotalLinesOfCode": { "type": "integer" },
                            "TotalChars": { "type": "integer" },
                            "TotalSizeBytes": { "type": "long" },
                            "TotalSizeHuman": { "type": "keyword" }
                        }
                    }
                }
            }
        });

        self.client.create_index(&self.name, mapping).await
    }

    /// Checks if the index exists.
    pub async fn exists(&self) -> Result<bool> {
        self.client.index_exists(&self.name).await
    }

    /// Deletes the index.
    pub async fn delete(&self) -> Result<()> {
        self.client.delete_index(&self.name).await
    }

    /// Bulk indexes multiple metadata documents.
    pub async fn bulk_index(&self, docs: &[&SourceMeta]) -> Result<()> {
        self.client.bulk_index(&self.name, docs).await
    }
}
