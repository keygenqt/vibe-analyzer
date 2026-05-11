//! Create project_meta index for project-level metadata.

use anyhow::Result;
use serde_json::json;

use crate::opensearch::indexes::index_client::OpenSearchIndexClient;

impl OpenSearchIndexClient {
    /// Create vibe_meta index with mapping for project metadata.
    pub async fn create_project_meta(&self) -> Result<()> {
        let mapping = json!({
            "settings": {
                "analysis": {
                    "analyzer": {
                        "russian_analyzer": { "type": "russian" },
                        "english_analyzer": { "type": "english" },
                        "chinese_analyzer": { "type": "chinese" }
                    }
                }
            },
            "mappings": {
                "dynamic": "strict",
                "properties": {
                    "path": { "type": "keyword" },
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
                            "preview": {
                                "type": "text",
                                "fields": {
                                    "russian": { "type": "text", "analyzer": "russian_analyzer" },
                                    "english": { "type": "text", "analyzer": "english_analyzer" },
                                    "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                                }
                            },
                            "frontmatter": { "type": "object", "dynamic": true },
                            "size_chars": { "type": "integer" },
                            "size_bytes": { "type": "long" }
                        }
                    },
                    "licenses": {
                        "properties": {
                            "path": { "type": "keyword" },
                            "spdx_id": { "type": "keyword" },
                            "confidence": { "type": "float" }
                        }
                    },
                    "stats": {
                        "properties": {
                            "total_files": { "type": "integer" },
                            "text_files": { "type": "integer" },
                            "binary_files": { "type": "integer" },
                            "total_chars": { "type": "integer" },
                            "total_size_bytes": { "type": "long" },
                            "total_size_lines": { "type": "integer" }
                        }
                    }
                }
            }
        });

        self.index_create(mapping).await
    }
}
