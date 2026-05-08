//! Classes, structs, and interfaces search functionality for OpenSearch indices.

use anyhow::Result;
use serde_json::json;

use crate::models::file_models::FileModels;
use crate::opensearch::tools::tool_client::OpenSearchToolClient;

impl OpenSearchToolClient {
    /// Searches for classes, structs, and interfaces across analysis indexes.
    pub async fn search_by_code_classes(
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
                                "path": "classes",
                                "query": {
                                    "bool": {
                                        "should": [
                                            { "match_phrase_prefix": { "classes.signature": { "query": q, "boost": 2.0 } } },
                                            { "match": { "classes.comments": { "query": q, "boost": 1.5, "fuzziness": "AUTO" } } }
                                        ]
                                    }
                                }
                            }},
                            { "nested": {
                                "path": "structs",
                                "query": {
                                    "bool": {
                                        "should": [
                                            { "match_phrase_prefix": { "structs.signature": { "query": q, "boost": 2.0 } } },
                                            { "match": { "structs.comments": { "query": q, "boost": 1.5, "fuzziness": "AUTO" } } }
                                        ]
                                    }
                                }
                            }},
                            { "nested": {
                                "path": "interfaces",
                                "query": {
                                    "bool": {
                                        "should": [
                                            { "match_phrase_prefix": { "interfaces.signature": { "query": q, "boost": 2.0 } } },
                                            { "match": { "interfaces.comments": { "query": q, "boost": 1.5, "fuzziness": "AUTO" } } }
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
                            { "match": { "tags": "classes structs interfaces" } }
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
