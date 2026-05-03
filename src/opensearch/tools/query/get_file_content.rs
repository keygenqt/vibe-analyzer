//! File content retrieval from OpenSearch files indexes.

use anyhow::Result;
use serde_json::json;

use crate::opensearch::tools::tools_client::OpenSearchToolsClient;

impl OpenSearchToolsClient {
    /// Get file content by root and path from files indexes.
    /// If root is None, searches across all provided indexes without root filter.
    pub async fn get_file_content(
        &self,
        indexes: &[String],
        root: Option<&str>,
        path: &str,
    ) -> Result<String> {
        let index_list = indexes.join(",");

        let mut must = vec![json!({
            "bool": {
                "should": [
                    { "term": { "path": path } },
                    { "wildcard": { "path": { "value": format!("*{}", path), "boost": 0.5 } } }
                ],
                "minimum_should_match": 1
            }
        })];

        if let Some(root) = root {
            must.insert(0, json!({ "term": { "root": root } }));
        }

        let query = json!({
            "size": 1,
            "query": {
                "bool": { "must": must }
            }
        });

        let resp = self.client.call(&index_list, query).await?;
        let hits = resp["hits"]["hits"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;

        hits.first()
            .and_then(|h| h["_source"]["content"].as_str())
            .map(|c| c.to_string())
            .ok_or_else(|| anyhow::anyhow!("File not found: {}", path))
    }
}
