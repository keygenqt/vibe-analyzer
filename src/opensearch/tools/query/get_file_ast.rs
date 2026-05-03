//! File AST retrieval from OpenSearch analysis indexes.

use anyhow::Result;
use serde_json::json;

use crate::opensearch::tools::tools_client::OpenSearchToolsClient;

impl OpenSearchToolsClient {
    /// Get file AST by root and path from analysis indexes.
    pub async fn get_file_ast(
        &self,
        indexes: &[String],
        root: Option<&str>,
        path: &str,
    ) -> Result<OpensearchFileMetaResponse> {
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
            .and_then(|h| serde_json::from_value(h["_source"].clone()).ok())
            .ok_or_else(|| anyhow::anyhow!("File not found: {}", path))
    }
}
