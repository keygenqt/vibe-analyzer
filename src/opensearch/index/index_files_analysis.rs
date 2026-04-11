//! Files analysis index management for AST and enrichment storage.
//! Handles creation and management of vibe_files_analysis_{hash} indices.

use crate::opensearch::client::OpenSearchClient;
use crate::opensearch::data::source_file_analysis::SourceFileAnalysis;
use crate::utils::app;
use anyhow::Result;
use serde_json::json;

/// Index handler for file analysis data.
pub struct FilesAnalysisIndex {
    client: OpenSearchClient,
    name: String,
}

impl FilesAnalysisIndex {
    /// Creates new FilesAnalysisIndex instance.
    pub fn new(client: OpenSearchClient, project_hash: &str) -> Self {
        Self {
            client,
            name: format!("{}-files-analysis-{}", app::app_name(), project_hash),
        }
    }

    /// Creates index with proper mapping.
    pub async fn create(&self) -> Result<()> {
        let mapping = json!({
            "mappings": {
                "properties": {
                    "Root": { "type": "keyword" },
                    "Path": { "type": "keyword" },
                    "Hash": { "type": "keyword" },
                    "Extension": { "type": "keyword" },
                    "IsText": { "type": "boolean" },
                    "SizeBytes": { "type": "long" },
                    "LinesOfCode": { "type": "integer" },
                    "SizeChars": { "type": "integer" },
                    "Ast": {
                        "properties": {
                            "HeaderComments": { "type": "text" },
                            "Imports": { "type": "keyword" },
                            "Variables": {
                                "type": "nested",
                                "properties": {
                                    "Signature": { "type": "text" },
                                    "Comments": { "type": "text" }
                                }
                            },
                            "Functions": {
                                "type": "nested",
                                "properties": {
                                    "Signature": { "type": "text" },
                                    "Comments": { "type": "text" }
                                }
                            },
                            "Classes": {
                                "type": "nested",
                                "properties": {
                                    "Signature": { "type": "text" },
                                    "Comments": { "type": "text" }
                                }
                            },
                            "Structs": {
                                "type": "nested",
                                "properties": {
                                    "Signature": { "type": "text" },
                                    "Comments": { "type": "text" }
                                }
                            },
                            "Enums": {
                                "type": "nested",
                                "properties": {
                                    "Signature": { "type": "text" },
                                    "Comments": { "type": "text" }
                                }
                            },
                            "Interfaces": {
                                "type": "nested",
                                "properties": {
                                    "Signature": { "type": "text" },
                                    "Comments": { "type": "text" }
                                }
                            },
                            "Frontmatter": {
                                "type": "object",
                                "dynamic": true
                            },
                            "Headings": {
                                "type": "nested",
                                "properties": {
                                    "Level": { "type": "integer" },
                                    "Title": { "type": "text" },
                                    "Preview": { "type": "text" }
                                }
                            },
                            "Links": {
                                "type": "nested",
                                "properties": {
                                    "Text": { "type": "text" },
                                    "Url": { "type": "keyword" }
                                }
                            },
                            "CodeBlocks": { "type": "keyword" }
                        }
                    },
                    "Description": { "type": "text" },
                    "Tags": { "type": "text" }
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
    pub async fn bulk_index(&self, docs: &[&SourceFileAnalysis]) -> Result<()> {
        self.client.bulk_index(&self.name, docs).await
    }
}
