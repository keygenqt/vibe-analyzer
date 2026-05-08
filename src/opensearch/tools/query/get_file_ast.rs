//! File AST retrieval from OpenSearch analysis indexes.

use anyhow::Result;
use serde_json::json;

use crate::models::file_models::FileModels;
use crate::opensearch::tools::tool_client::OpenSearchToolClient;

impl OpenSearchToolClient {
    /// Get file AST by path from analysis indexes.
    pub async fn get_file_ast(&self, path: &str) -> Result<FileModels> {
        let query = json!({
            "size": 1,
            "query": {
                "bool": {
                    "filter": [
                        { "term": { "type": "language" } }
                    ],
                    "should": [
                        { "term": { "path": path } },
                        { "wildcard": { "path": { "value": format!("*{}", path), "boost": 0.5 } } }
                    ],
                    "minimum_should_match": 1
                }
            }
        });

        let resp = self.call(query).await?;
        let hits = resp["hits"]["hits"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;

        hits.first()
            .and_then(|h| serde_json::from_value(h["_source"].clone()).ok())
            .ok_or_else(|| anyhow::anyhow!("File not found: {}", path))
    }
}
