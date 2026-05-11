//! Enrichment check query.
//! Checks if enrichment data exists in analysis indices.

use anyhow::Result;
use serde_json::{Value, json};

use crate::opensearch::tools::tool_client::OpenSearchToolClient;

impl OpenSearchToolClient {
    /// Check if any documents have enrichment data across all analysis indices.
    pub async fn is_enrich(&self) -> Result<bool> {
        if self.names.is_empty() {
            return Ok(false);
        }
        let query = json!({
            "query": { "exists": { "field": "summary" } },
            "size": 1
        });
        let resp: Value = self.call(query).await?;
        Ok(resp["hits"]["total"]["value"].as_u64().unwrap_or(0) > 0)
    }
}
