//! Search functionality for OpenSearch indices.
//! Provides meta, ast, file, and get queries.

use anyhow::Result;
use serde_json::Value;
use serde_json::json;

use crate::opensearch::client::OpenSearchClient;
use crate::opensearch::data::requests::source_file_analysis::SourceFileAnalysis;
use crate::opensearch::data::responses::search_ast::SearchAst;
use crate::opensearch::data::responses::search_file_hash::SearchFileHash;
use crate::opensearch::data::responses::search_hit::SearchHit;

pub struct SearchClient {
    client: OpenSearchClient,
}

impl SearchClient {
    pub fn new(client: OpenSearchClient) -> Self {
        Self { client }
    }

    /// Get all file hashes from a files index using scroll API.
    pub async fn get_hashes(&self, index: &str) -> Result<Vec<SearchFileHash>> {
        let mut all_results = Vec::new();

        // Initial search with scroll
        let url = format!("{}/{}/_search?scroll=1m", self.client.base_url(), index);
        let query = json!({
            "size": 1000,
            "_source": ["Root", "Path", "Hash"],
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
                    h["_source"]["Root"].as_str(),
                    h["_source"]["Path"].as_str(),
                    h["_source"]["Hash"].as_str(),
                ) {
                    all_results.push(SearchFileHash {
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

    /// Search in AST elements.
    pub async fn search_ast(
        &self,
        indexes: &[String],
        pattern: &str,
        limit: usize,
    ) -> Result<Vec<SearchAst>> {
        // Join indexes with comma for multi-index search
        let index_list = indexes.join(",");
        // Query to OpenSearch
        let query = json!({
            "size": limit,
            "query": {
                "bool": {
                    "should": [
                        {
                            "match": {
                                "Summary": {
                                    "query": pattern,
                                    "boost": 4.5
                                }
                            }
                        },
                        {
                            "match": {
                                "Path": {
                                    "query": pattern,
                                    "boost": 5.0
                                }
                            }
                        },
                        {
                            "match": {
                                "Tags": {
                                    "query": pattern,
                                    "boost": 5.0
                                }
                            }
                        },
                        {
                            "match": {
                                "Description": {
                                    "query": pattern,
                                    "boost": 4.0
                                }
                            }
                        },
                        {
                            "match": {
                                "Ast.HeaderComments": {
                                    "query": pattern,
                                    "boost": 3.5
                                }
                            }
                        },
                        {
                            "nested": {
                                "path": "Ast.Functions",
                                "query": {
                                    "bool": {
                                        "should": [
                                            {
                                                "match": {
                                                    "Ast.Functions.Signature": {
                                                        "query": pattern,
                                                        "boost": 3.0
                                                    }
                                                }
                                            },
                                            {
                                                "match": {
                                                    "Ast.Functions.Comments": {
                                                        "query": pattern,
                                                        "boost": 2.0
                                                    }
                                                }
                                            }
                                        ]
                                    }
                                }
                            }
                        },
                        {
                            "nested": {
                                "path": "Ast.Classes",
                                "query": {
                                    "bool": {
                                        "should": [
                                            {
                                                "match": {
                                                    "Ast.Classes.Signature": {
                                                        "query": pattern,
                                                        "boost": 3.0
                                                    }
                                                }
                                            },
                                            {
                                                "match": {
                                                    "Ast.Classes.Comments": {
                                                        "query": pattern,
                                                        "boost": 2.0
                                                    }
                                                }
                                            }
                                        ]
                                    }
                                }
                            }
                        },
                        {
                            "nested": {
                                "path": "Ast.Structs",
                                "query": {
                                    "bool": {
                                        "should": [
                                            {
                                                "match": {
                                                    "Ast.Structs.Signature": {
                                                        "query": pattern,
                                                        "boost": 2.5
                                                    }
                                                }
                                            },
                                            {
                                                "match": {
                                                    "Ast.Structs.Comments": {
                                                        "query": pattern,
                                                        "boost": 1.5
                                                    }
                                                }
                                            }
                                        ]
                                    }
                                }
                            }
                        },
                        {
                            "nested": {
                                "path": "Ast.Interfaces",
                                "query": {
                                    "bool": {
                                        "should": [
                                            {
                                                "match": {
                                                    "Ast.Interfaces.Signature": {
                                                        "query": pattern,
                                                        "boost": 2.5
                                                    }
                                                }
                                            },
                                            {
                                                "match": {
                                                    "Ast.Interfaces.Comments": {
                                                        "query": pattern,
                                                        "boost": 1.5
                                                    }
                                                }
                                            }
                                        ]
                                    }
                                }
                            }
                        },
                        {
                            "nested": {
                                "path": "Ast.Enums",
                                "query": {
                                    "bool": {
                                        "should": [
                                            {
                                                "match": {
                                                    "Ast.Enums.Signature": {
                                                        "query": pattern,
                                                        "boost": 2.0
                                                    }
                                                }
                                            },
                                            {
                                                "match": {
                                                    "Ast.Enums.Comments": {
                                                        "query": pattern,
                                                        "boost": 1.5
                                                    }
                                                }
                                            }
                                        ]
                                    }
                                }
                            }
                        },
                        {
                            "nested": {
                                "path": "Ast.Variables",
                                "query": {
                                    "match": {
                                        "Ast.Variables.Signature": {
                                            "query": pattern,
                                            "boost": 1.0
                                        }
                                    }
                                }
                            }
                        },
                        {
                            "match": {
                                "Ast.Imports": {
                                    "query": pattern,
                                    "boost": 1.0
                                }
                            }
                        },
                        {
                            "nested": {
                                "path": "Ast.Headings",
                                "query": {
                                    "match": {
                                        "Ast.Headings.Title": {
                                            "query": pattern,
                                            "boost": 1.5
                                        }
                                    }
                                }
                            }
                        },
                        {
                            "nested": {
                                "path": "Ast.Links",
                                "query": {
                                    "match": {
                                        "Ast.Links.Text": {
                                            "query": pattern,
                                            "boost": 1.0
                                        }
                                    }
                                }
                            }
                        },
                        {
                            "match": {
                                "Ast.Frontmatter": {
                                    "query": pattern,
                                    "boost": 2.0
                                }
                            }
                        }
                    ],
                    "minimum_should_match": 1
                }
            }
        });

        let resp = self.client.search(&index_list, query).await?;
        let hits = resp["hits"]["hits"].as_array().unwrap();

        let results: Vec<SearchAst> = hits
            .iter()
            .filter_map(|h| {
                let analysis: SourceFileAnalysis =
                    serde_json::from_value(h["_source"].clone()).ok()?;
                let ast = analysis.ast.as_ref()?;
                let index = h["_index"].as_str().unwrap_or("").to_string();

                Some(SearchAst {
                    hit: SearchHit {
                        root: analysis.root,
                        index,
                        path: analysis.path.clone(),
                    },
                    path: analysis.path,
                    header_comments: ast.header_comments.clone(),
                    imports: ast.imports.clone(),
                    variables: ast.variables.clone(),
                    functions: ast.functions.clone(),
                    classes: ast.classes.clone(),
                    structs: ast.structs.clone(),
                    enums: ast.enums.clone(),
                    interfaces: ast.interfaces.clone(),
                    frontmatter: ast.frontmatter.clone(),
                    headings: ast.headings.clone(),
                    links: ast.links.clone(),
                    code_blocks: ast.code_blocks.clone(),
                })
            })
            .collect();

        Ok(results)
    }
}
