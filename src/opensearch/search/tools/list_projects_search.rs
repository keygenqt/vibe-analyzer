//! Search functionality for OpenSearch indices.
//! Provides meta, ast, file, and get queries.

use anyhow::Result;
use serde_json::json;

use crate::configs::app::AppConfig;
use crate::opensearch::data::responses::opensearch_project_meta_response::OpensearchProjectMetaResponse;
use crate::opensearch::search::search_client::SearchClient;

impl SearchClient {
    /// Returns metadata for all indexed projects.
    pub async fn list_projects(&self) -> Result<Vec<OpensearchProjectMetaResponse>> {
        let index = AppConfig::meta_index();
        let query = json!({
            "size": 999,
            "query": { "match_all": {} }
        });
        let resp = self.client.search(&index, query).await?;
        let hits = resp["hits"]["hits"].as_array().unwrap();
        let results: Vec<OpensearchProjectMetaResponse> = hits
            .iter()
            .filter_map(|h| serde_json::from_value(h["_source"].clone()).ok())
            .collect();

        Ok(results)
    }
}
