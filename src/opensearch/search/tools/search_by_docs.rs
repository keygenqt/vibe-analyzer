//! Documentation search functionality for OpenSearch indices.

use anyhow::Result;
use serde_json::json;

use crate::opensearch::data::requests::opensearch_file_stats_request::OpensearchFileStatsRequest;
use crate::opensearch::data::responses::opensearch_file_meta_response::OpensearchFileMetaResponse;
use crate::opensearch::search::search_client::SearchClient;

impl SearchClient {
    pub async fn search_by_docs(
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
                            { "term": { "ast.frontmatter.knowledge": { "value": "true", "boost": 5.0 } } },
                            { "nested": {
                                "path": "ast.headings",
                                "query": {
                                    "bool": {
                                        "should": [
                                            { "wildcard": { "ast.headings.title.russian": { "value": format!("*{}*", q), "boost": 8.0 } } },
                                            { "wildcard": { "ast.headings.title.english": { "value": format!("*{}*", q), "boost": 8.0 } } },
                                            { "wildcard": { "ast.headings.title.chinese": { "value": format!("*{}*", q), "boost": 8.0 } } },
                                            { "match": { "ast.headings.title.russian": { "query": q, "operator": "or", "boost": 4.0, "fuzziness": "AUTO" } } },
                                            { "match": { "ast.headings.title.english": { "query": q, "operator": "or", "boost": 4.0, "fuzziness": "AUTO" } } },
                                            { "match": { "ast.headings.title.chinese": { "query": q, "operator": "or", "boost": 4.0, "fuzziness": "AUTO" } } },
                                            { "wildcard": { "ast.headings.preview.russian": { "value": format!("*{}*", q), "boost": 3.0 } } },
                                            { "wildcard": { "ast.headings.preview.english": { "value": format!("*{}*", q), "boost": 3.0 } } },
                                            { "wildcard": { "ast.headings.preview.chinese": { "value": format!("*{}*", q), "boost": 3.0 } } },
                                            { "match": { "ast.headings.preview.russian": { "query": q, "operator": "or", "boost": 2.0, "fuzziness": "AUTO" } } },
                                            { "match": { "ast.headings.preview.english": { "query": q, "operator": "or", "boost": 2.0, "fuzziness": "AUTO" } } },
                                            { "match": { "ast.headings.preview.chinese": { "query": q, "operator": "or", "boost": 2.0, "fuzziness": "AUTO" } } }
                                        ]
                                    }
                                }
                            }},
                            { "nested": {
                                "path": "ast.links",
                                "query": {
                                    "bool": {
                                        "should": [
                                            { "wildcard": { "ast.links.text.russian": { "value": format!("*{}*", q), "boost": 3.0 } } },
                                            { "wildcard": { "ast.links.text.english": { "value": format!("*{}*", q), "boost": 3.0 } } },
                                            { "wildcard": { "ast.links.text.chinese": { "value": format!("*{}*", q), "boost": 3.0 } } },
                                            { "match": { "ast.links.text.russian": { "query": q, "operator": "or", "boost": 2.0, "fuzziness": "AUTO" } } },
                                            { "match": { "ast.links.text.english": { "query": q, "operator": "or", "boost": 2.0, "fuzziness": "AUTO" } } },
                                            { "match": { "ast.links.text.chinese": { "query": q, "operator": "or", "boost": 2.0, "fuzziness": "AUTO" } } },
                                            { "term": { "ast.links.url": { "value": q, "boost": 1.5 } } }
                                        ]
                                    }
                                }
                            }},
                            { "match": { "ast.code_blocks": { "query": q, "boost": 1.5 } } },
                            { "match": { "tags.russian": { "query": q, "boost": 1.0 } } },
                            { "match": { "tags.english": { "query": q, "boost": 1.0 } } },
                            { "match": { "tags.chinese": { "query": q, "boost": 1.0 } } }
                        ],
                        "minimum_should_match": 1
                    }
                }
            }),
            None => json!({
                "size": limit,
                "query": {
                    "bool": {
                        "should": [
                            { "exists": { "field": "ast.headings" } },
                            { "exists": { "field": "ast.links" } },
                            { "exists": { "field": "ast.frontmatter" } }
                        ],
                        "minimum_should_match": 1
                    }
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
