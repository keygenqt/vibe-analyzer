//! Create file_meta index for AST and enrichment data.

use anyhow::Result;
use serde_json::json;

use crate::opensearch::indexes::indexes_client::OpenSearchIndexesClient;

impl OpenSearchIndexesClient {
    /// Create vibe_files_analysis_{hash} index with mapping for AST and enrichment.
    pub async fn create_file_meta(&self) -> Result<()> {
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
                    // Common
                    "path": { "type": "keyword" },
                    "type": { "type": "keyword" },
                    "size_bytes": { "type": "long" },

                    // Language
                    "language": { "type": "keyword" },
                    "summary": {
                        "type": "text",
                        "fields": {
                            "russian": { "type": "text", "analyzer": "russian_analyzer" },
                            "english": { "type": "text", "analyzer": "english_analyzer" },
                            "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                        }
                    },
                    "lines_of_code": { "type": "integer" },
                    "tags": {
                        "type": "text",
                        "fields": {
                            "russian": { "type": "text", "analyzer": "russian_analyzer" },
                            "english": { "type": "text", "analyzer": "english_analyzer" },
                            "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                        }
                    },
                    "functions": {
                        "type": "nested",
                        "properties": {
                            "signature": { "type": "text" },
                            "comments": { "type": "text" }
                        }
                    },
                    "classes": {
                        "type": "nested",
                        "properties": {
                            "signature": { "type": "text" },
                            "comments": { "type": "text" }
                        }
                    },
                    "structs": {
                        "type": "nested",
                        "properties": {
                            "signature": { "type": "text" },
                            "comments": { "type": "text" }
                        }
                    },
                    "enums": {
                        "type": "nested",
                        "properties": {
                            "signature": { "type": "text" },
                            "comments": { "type": "text" }
                        }
                    },
                    "interfaces": {
                        "type": "nested",
                        "properties": {
                            "signature": { "type": "text" },
                            "comments": { "type": "text" }
                        }
                    },
                    "variables": {
                        "type": "nested",
                        "properties": {
                            "signature": { "type": "text" },
                            "comments": { "type": "text" }
                        }
                    },
                    "imports": { "type": "keyword" },
                    "header_comments": { "type": "text" },
                    "headings": {
                        "type": "nested",
                        "properties": {
                            "level": { "type": "integer" },
                            "title": {
                                "type": "text",
                                "fields": {
                                    "keyword": { "type": "keyword" }
                                }
                            },
                            "preview": {
                                "type": "text",
                                "fields": {
                                    "keyword": { "type": "keyword" }
                                }
                            }
                        }
                    },
                    "links": {
                        "type": "nested",
                        "properties": {
                            "text": { "type": "keyword" },
                            "url": { "type": "keyword" }
                        }
                    },
                    "code_blocks": { "type": "keyword" },
                    "frontmatter": { "type": "object", "dynamic": true },

                    // Text
                    "preview": { "type": "text" },
                    "extension": { "type": "keyword" },
                }
            }
        });

        self.index_create(mapping).await
    }
}
