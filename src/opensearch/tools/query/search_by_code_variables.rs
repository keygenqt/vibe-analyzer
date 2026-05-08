//! Variables and enums search functionality for OpenSearch indices.

use anyhow::Result;
use serde_json::json;

use crate::models::file_models::FileModels;
use crate::opensearch::tools::tool_client::OpenSearchToolClient;

impl OpenSearchToolClient {
    /// Searches for variables and enums across analysis indexes.
    pub async fn search_by_code_variables(
        &self,
        query: Option<String>,
        limit: usize,
    ) -> Result<Vec<FileModels>> {
        let search_query = match query {
            Some(q) => json!({
                "size": limit,
                "query": {
                    "bool": {
                        "filter": [
                            { "term": { "type": "language" } }
                        ],
                        "should": [
                            { "match": { "tags": { "query": q, "boost": 4.0 } } },
                            { "match": { "ast.tags": { "query": q, "boost": 3.0 } } },
                            { "nested": {
                                "path": "variables",
                                "query": {
                                    "bool": {
                                        "should": [
                                            { "match": { "variables.signature": { "query": q, "boost": 2.0, "fuzziness": "AUTO" } } },
                                            { "match": { "variables.comments": { "query": q, "boost": 1.5 } } }
                                        ]
                                    }
                                }
                            }},
                            { "nested": {
                                "path": "enums",
                                "query": {
                                    "bool": {
                                        "should": [
                                            { "match": { "enums.signature": { "query": q, "boost": 2.0, "fuzziness": "AUTO" } } },
                                            { "match": { "enums.comments": { "query": q, "boost": 1.5 } } }
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
                    "bool": {
                        "filter": [
                            { "term": { "type": "language" } }
                        ],
                        "must": [
                            { "match": { "tags": "variables enums" } }
                        ]
                    }
                }
            }),
        };

        let resp = self.call(search_query).await?;
        let hits = resp["hits"]["hits"].as_array().unwrap();

        let results: Vec<FileModels> = hits
            .iter()
            .filter_map(|h| serde_json::from_value(h["_source"].clone()).ok())
            .collect();

        Ok(results)
    }
}
