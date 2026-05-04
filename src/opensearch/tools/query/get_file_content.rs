//! File content retrieval from OpenSearch files indexes.

use anyhow::Result;
use serde_json::json;

use crate::opensearch::tools::tool_client::OpenSearchToolClient;

impl OpenSearchToolClient {
    /// Get file content by path from files indexes.
    pub async fn get_file_content(&self, path: &str) -> Result<(String, String)> {
        let query = json!({
            "size": 1,
            "query": {
                "bool": {
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

        let hit = hits
            .first()
            .ok_or_else(|| anyhow::anyhow!("File not found: {}", path))?;

        let file_path = hit["_source"]["path"].as_str().unwrap_or(path).to_string();
        let content = hit["_source"]["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Content not found for: {}", path))?
            .to_string();

        Ok((file_path, content))
    }
}
