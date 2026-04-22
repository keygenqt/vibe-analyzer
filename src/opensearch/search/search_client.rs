//! Search functionality for OpenSearch indices.
//! Provides meta, ast, file, and get queries.

use anyhow::Result;
use serde_json::Value;
use serde_json::json;

use crate::opensearch::client::OpenSearchClient;
use crate::opensearch::data::responses::opensearch_list_hashes_response::OpensearchHashResponse;

/// OpenSearch client wrapper.
#[derive(Clone)]
pub struct SearchClient {
    pub client: OpenSearchClient,
}

impl SearchClient {
    /// Creates new SearchClient.
    pub fn new(client: OpenSearchClient) -> Self {
        Self { client }
    }

    /// Get all file hashes from a files index using scroll API.
    pub async fn get_hashes(&self, index: &str) -> Result<Vec<OpensearchHashResponse>> {
        // Check if index exists
        if !self.client.index_exists(index).await? {
            return Ok(Vec::new());
        }

        let mut all_results = Vec::new();

        // Initial search with scroll
        let url = format!("{}/{}/_search?scroll=1m", self.client.base_url(), index);
        let query = json!({
            "size": 1000,
            "_source": ["root", "path", "hash"],
            "query": { "match_all": {} }
        });

        let mut resp: Value = self
            .client
            .post(&url)
            .json(&query)
            .send()
            .await?
            .json()
            .await?;

        // Collect results and scroll
        loop {
            let hits = resp["hits"]["hits"].as_array().unwrap();
            if hits.is_empty() {
                break;
            }

            for h in hits {
                if let (Some(root), Some(path), Some(hash)) = (
                    h["_source"]["root"].as_str(),
                    h["_source"]["path"].as_str(),
                    h["_source"]["hash"].as_str(),
                ) {
                    all_results.push(OpensearchHashResponse {
                        path: format!("{}/{}", root, path),
                        hash: hash.to_string(),
                    });
                }
            }

            // Get next batch
            let scroll_id = resp["_scroll_id"].as_str().unwrap();
            resp = self
                .client
                .post(&format!("{}/_search/scroll", self.client.base_url()))
                .json(&json!({
                    "scroll": "1m",
                    "scroll_id": scroll_id
                }))
                .send()
                .await?
                .json()
                .await?;
        }

        Ok(all_results)
    }
}
