//! Import search functionality for OpenSearch indices.

use anyhow::Result;
use serde_json::json;

use crate::opensearch::data::requests::opensearch_file_stats_request::OpensearchFileStatsRequest;
use crate::opensearch::data::responses::opensearch_file_meta_response::OpensearchFileMetaResponse;
use crate::opensearch::search::search_client::SearchClient;

impl SearchClient {
    /// Search for imports across multiple indexes.
    /// If query is None, returns all files with imports.
    pub async fn search_by_code_imports(
        &self,
        indexes: &[String],
        query: Option<String>,
        limit: usize,
    ) -> Result<Vec<OpensearchFileMetaResponse>> {
        let index_list = indexes.join(",");

        let search_query = match query {
            Some(q) => json!({
                "size": limit,
                "query": {
                    "bool": {
                        "should": [
                            { "match": { "tags": { "query": q, "boost": 4.0 } } },
                            { "match": { "ast.tags": { "query": q, "boost": 3.0 } } },
                            { "match": { "ast.imports": { "query": q, "boost": 2.0, "fuzziness": "AUTO" } } }
                        ],
                        "minimum_should_match": 1
                    }
                }
            }),
            None => json!({
                "size": limit,
                "query": {
                    "match": { "ast.tags": "imports" }
                }
            }),
        };

        let resp = self.client.search(&index_list, search_query).await?;
        let hits = resp["hits"]["hits"].as_array().unwrap();

        let results: Vec<OpensearchFileMetaResponse> = hits
            .iter()
            .filter_map(|h| {
                let source: OpensearchFileStatsRequest =
                    serde_json::from_value(h["_source"].clone()).ok()?;
                Some(OpensearchFileMetaResponse::from(source))
            })
            .collect();

        Ok(results)
    }
}
