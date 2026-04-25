//! Files analysis index management for AST and enrichment storage.
//! Handles creation and management of vibe_files_analysis_{hash} indices.

use std::path::PathBuf;

use crate::configs::app::AppConfig;
use crate::opensearch::client::OpenSearchClient;
use crate::opensearch::data::requests::opensearch_file_stats_request::OpensearchFileStatsRequest;
use anyhow::Result;
use serde_json::json;

/// Index handler for file analysis data.
pub struct FilesAnalysisIndex {
    client: OpenSearchClient,
    name: String,
}

impl FilesAnalysisIndex {
    /// Creates new FilesAnalysisIndex instance.
    pub fn new(client: OpenSearchClient, source: &PathBuf) -> Self {
        Self {
            client,
            name: AppConfig::files_analysis_index(source),
        }
    }

    /// Creates index with proper mapping.
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
                    "path": { "type": "keyword" },
                    "hash": { "type": "keyword" },
                    "extension": { "type": "keyword" },
                    "is_text": { "type": "boolean" },
                    "size_bytes": { "type": "long" },
                    "lines_of_code": { "type": "integer" },
                    "size_chars": { "type": "integer" },
                    "ast": {
                        "properties": {
                            "header_comments": {
                                "type": "text",
                                "fields": {
                                    "russian": { "type": "text", "analyzer": "russian_analyzer" },
                                    "english": { "type": "text", "analyzer": "english_analyzer" },
                                    "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                                }
                            },
                            "imports": { "type": "text" },
                            "variables": {
                                "type": "nested",
                                "properties": {
                                    "signature": { "type": "text" },
                                    "comments": {
                                        "type": "text",
                                        "fields": {
                                            "russian": { "type": "text", "analyzer": "russian_analyzer" },
                                            "english": { "type": "text", "analyzer": "english_analyzer" },
                                            "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                                        }
                                    }
                                }
                            },
                            "functions": {
                                "type": "nested",
                                "properties": {
                                    "signature": { "type": "text" },
                                    "comments": {
                                        "type": "text",
                                        "fields": {
                                            "russian": { "type": "text", "analyzer": "russian_analyzer" },
                                            "english": { "type": "text", "analyzer": "english_analyzer" },
                                            "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                                        }
                                    }
                                }
                            },
                            "classes": {
                                "type": "nested",
                                "properties": {
                                    "signature": { "type": "text" },
                                    "comments": {
                                        "type": "text",
                                        "fields": {
                                            "russian": { "type": "text", "analyzer": "russian_analyzer" },
                                            "english": { "type": "text", "analyzer": "english_analyzer" },
                                            "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                                        }
                                    }
                                }
                            },
                            "structs": {
                                "type": "nested",
                                "properties": {
                                    "signature": { "type": "text" },
                                    "comments": {
                                        "type": "text",
                                        "fields": {
                                            "russian": { "type": "text", "analyzer": "russian_analyzer" },
                                            "english": { "type": "text", "analyzer": "english_analyzer" },
                                            "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                                        }
                                    }
                                }
                            },
                            "enums": {
                                "type": "nested",
                                "properties": {
                                    "signature": { "type": "text" },
                                    "comments": {
                                        "type": "text",
                                        "fields": {
                                            "russian": { "type": "text", "analyzer": "russian_analyzer" },
                                            "english": { "type": "text", "analyzer": "english_analyzer" },
                                            "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                                        }
                                    }
                                }
                            },
                            "interfaces": {
                                "type": "nested",
                                "properties": {
                                    "signature": { "type": "text" },
                                    "comments": {
                                        "type": "text",
                                        "fields": {
                                            "russian": { "type": "text", "analyzer": "russian_analyzer" },
                                            "english": { "type": "text", "analyzer": "english_analyzer" },
                                            "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                                        }
                                    }
                                }
                            },
                            "frontmatter": {
                                "type": "object",
                                "dynamic": true
                            },
                            "headings": {
                                "type": "nested",
                                "properties": {
                                    "level": { "type": "integer" },
                                    "title": {
                                        "type": "text",
                                        "fields": {
                                            "keyword": { "type": "keyword" },
                                            "russian": { "type": "text", "analyzer": "russian_analyzer" },
                                            "english": { "type": "text", "analyzer": "english_analyzer" },
                                            "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                                        }
                                    },
                                    "preview": {
                                        "type": "text",
                                        "fields": {
                                            "keyword": { "type": "keyword" },
                                            "russian": { "type": "text", "analyzer": "russian_analyzer" },
                                            "english": { "type": "text", "analyzer": "english_analyzer" },
                                            "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                                        }
                                    }
                                }
                            },
                            "links": {
                                "type": "nested",
                                "properties": {
                                    "text": {
                                        "type": "text",
                                        "fields": {
                                            "russian": { "type": "text", "analyzer": "russian_analyzer" },
                                            "english": { "type": "text", "analyzer": "english_analyzer" },
                                            "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                                        }
                                    },
                                    "url": { "type": "keyword" }
                                }
                            },
                            "code_blocks": { "type": "keyword" },
                            "tags": {
                                "type": "text",
                                "fields": {
                                    "russian": { "type": "text", "analyzer": "russian_analyzer" },
                                    "english": { "type": "text", "analyzer": "english_analyzer" },
                                    "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                                }
                            }
                        }
                    },
                    "description": {
                        "type": "text",
                        "fields": {
                            "russian": { "type": "text", "analyzer": "russian_analyzer" },
                            "english": { "type": "text", "analyzer": "english_analyzer" },
                            "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                        }
                    },
                    "tags": {
                        "type": "text",
                        "fields": {
                            "russian": { "type": "text", "analyzer": "russian_analyzer" },
                            "english": { "type": "text", "analyzer": "english_analyzer" },
                            "chinese": { "type": "text", "analyzer": "chinese_analyzer" }
                        }
                    }
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
    pub async fn bulk_index(&self, docs: &[&OpensearchFileStatsRequest]) -> Result<()> {
        self.client.bulk_index(&self.name, docs).await
    }
}
