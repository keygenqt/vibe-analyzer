//! Metadata index management for project-level information.
//! Handles creation and management of the vibe_meta index.

use crate::configs::app::AppConfig;
use crate::opensearch::client::OpenSearchClient;
use crate::opensearch::data::requests::source_meta::SourceMeta;
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
            name: AppConfig::meta_index(),
        }
    }

    /// Creates the index with proper mapping.
    pub async fn create(&self) -> Result<()> {
        let mapping = json!({
            "settings": {
                "analysis": {
                    "analyzer": {
                        "russian_analyzer": {
                            "type": "russian"
                        },
                        "english_analyzer": {
                            "type": "english"
                        },
                        "chinese_analyzer": {
                            "type": "chinese"
                        }
                    }
                }
            },
            "mappings": {
                "dynamic": "strict",
                "properties": {
                    "root": { "type": "keyword" },
                    "summary": {
                        "type": "text",
                        "fields": {
                            "russian": { "type": "text", "analyzer": "russian_analyzer" },
                            "english": { "type": "text", "analyzer": "english_analyzer" },
                            "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                        }
                    },
                    "readme": {
                        "properties": {
                            "path": { "type": "keyword" },
                            "content": {
                                "type": "text",
                                "store": true,
                                "fields": {
                                    "russian": { "type": "text", "analyzer": "russian_analyzer", "store": true },
                                    "english": { "type": "text", "analyzer": "english_analyzer", "store": true },
                                    "chinese": { "type": "text", "analyzer": "chinese_analyzer", "store": true }
                                }
                            },
                            "size_bytes": { "type": "long" }
                        }
                    },
                    "license": {
                        "properties": {
                            "spdx_id": { "type": "keyword" },
                            "name": { "type": "text" },
                            "confidence": { "type": "float" },
                            "path": { "type": "keyword" }
                        }
                    },
                    "stats": {
                        "properties": {
                            "total_files": { "type": "integer" },
                            "text_files": { "type": "integer" },
                            "binary_files": { "type": "integer" },
                            "total_functions": { "type": "integer" },
                            "total_classes": { "type": "integer" },
                            "total_structs": { "type": "integer" },
                            "total_enums": { "type": "integer" },
                            "total_interfaces": { "type": "integer" },
                            "total_imports": { "type": "integer" },
                            "total_lines_of_code": { "type": "integer" },
                            "total_chars": { "type": "integer" },
                            "total_size_bytes": { "type": "long" },
                            "total_size_human": { "type": "keyword" }
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
