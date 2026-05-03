//! Functions and methods search functionality for OpenSearch indices.

use anyhow::Result;
use serde_json::json;

use crate::opensearch::tools::tools_client::OpenSearchToolsClient;

impl OpenSearchToolsClient {
    pub async fn search_by_code_functions(
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
                                "path": "ast.functions",
                                "query": {
                                    "bool": {
                                        "should": [
                                            { "match_phrase_prefix": { "ast.functions.signature": { "query": q, "boost": 2.0 } } },
                                            { "match": { "ast.functions.comments": { "query": q, "boost": 1.5, "fuzziness": "AUTO" } } }
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
                    "match": { "ast.tags": "functions" }
                }
            }),
        };

        let resp = self.client.call(&index_list, search_query).await?;
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
