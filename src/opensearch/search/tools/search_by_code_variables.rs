//! Variables and enums search functionality for OpenSearch indices.

use anyhow::Result;
use serde_json::json;

use crate::opensearch::data::requests::opensearch_file_stats_request::OpensearchFileStatsRequest;
use crate::opensearch::data::responses::opensearch_file_meta_response::OpensearchFileMetaResponse;
use crate::opensearch::search::search_client::SearchClient;

impl SearchClient {
    pub async fn search_by_code_variables(
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
                            { "nested": {
                                "path": "ast.variables",
                                "query": {
                                    "bool": {
                                        "should": [
                                            { "match": { "ast.variables.signature": { "query": q, "boost": 2.0, "fuzziness": "AUTO" } } },
                                            { "match": { "ast.variables.comments": { "query": q, "boost": 1.5 } } }
                                        ]
                                    }
                                }
                            }},
                            { "nested": {
                                "path": "ast.enums",
                                "query": {
                                    "bool": {
                                        "should": [
                                            { "match": { "ast.enums.signature": { "query": q, "boost": 2.0, "fuzziness": "AUTO" } } },
                                            { "match": { "ast.enums.comments": { "query": q, "boost": 1.5 } } }
                                        ]
                                    }
                                }
                            }}
                        ],
                        "minimum_should_match": 1
                    }
                }
            }),
            None => json!({
                "size": limit,
                "query": {
                    "match": { "ast.tags": "variables enums" }
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
