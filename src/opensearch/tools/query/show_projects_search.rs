//! Search functionality for OpenSearch indices.
//! Provides meta, ast, file, and get queries.

use anyhow::Result;
use serde_json::json;

use crate::opensearch::tools::tools_client::OpenSearchToolsClient;

use crate::configs::app::AppConfig;

impl OpenSearchToolsClient {
    /// Returns metadata for all indexed projects.
    pub async fn show_projects_search(&self) -> Result<Vec<OpensearchProjectMetaResponse>> {
        let index = AppConfig::meta_index();
        let query = json!({
            "size": 999,
            "query": { "match_all": {} }
        });
        let resp = self.client.call(&index, query).await?;
        let hits = resp["hits"]["hits"].as_array().unwrap();
        let results: Vec<OpensearchProjectMetaResponse> = hits
            .iter()
            .filter_map(|h| serde_json::from_value(h["_source"].clone()).ok())
            .collect();

        Ok(results)
    }
}
