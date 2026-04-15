//! Search functionality for OpenSearch indices.
//! Provides meta, ast, file, and get queries.

use std::collections::HashMap;

use anyhow::Result;
use serde_json::Value;
use serde_json::json;

use crate::configs::app::AppConfig;
use crate::opensearch::client::OpenSearchClient;
use crate::opensearch::data::requests::source_file_analysis::SourceFileAnalysis;
use crate::opensearch::data::responses::search_ast::SearchAst;
use crate::opensearch::data::responses::search_file::SearchFile;
use crate::opensearch::data::responses::search_file_hash::SearchFileHash;
use crate::opensearch::data::responses::search_hit::SearchHit;
use crate::opensearch::data::responses::search_meta::SearchMeta;
use crate::opensearch::data::responses::search_stats::LanguageStat;
use crate::opensearch::data::responses::search_stats::SearchStats;
use crate::opensearch::data::responses::search_tree::SearchTree;
use crate::scanner::data::output::source_stats::SourceStats;

#[derive(Clone)]
pub struct SearchClient {
    client: OpenSearchClient,
}

impl SearchClient {
    pub fn new(client: OpenSearchClient) -> Self {
        Self { client }
    }

    /// Get statistics for multiple sources.
    pub async fn get_stats(&self, indexes: &[String]) -> Result<SearchStats> {
        let mut lang_map: HashMap<String, LanguageStat> = HashMap::new();
        let mut other_stat = LanguageStat {
            language: "Other".to_string(),
            files: 0,
            lines: 0,
            ast_objects: 0,
            size_bytes: 0,
            size_human: String::new(),
        };

        for index in indexes {
            let all_docs = self.get_all_ast(index).await?;

            for doc in all_docs {
                if !doc.is_text {
                    continue;
                }

                let ext = doc.path.split('.').last().unwrap_or("");
                let language = crate::scanner::ast::parser::get_language_name(ext);

                if let Some(lang_name) = language {
                    let entry = lang_map
                        .entry(lang_name.to_string())
                        .or_insert(LanguageStat {
                            language: lang_name.to_string(),
                            files: 0,
                            lines: 0,
                            ast_objects: 0,
                            size_bytes: 0,
                            size_human: String::new(),
                        });

                    entry.files += 1;
                    entry.lines += doc.lines_of_code.unwrap_or(0);
                    entry.size_bytes += doc.size_bytes;

                    if let Some(ast) = &doc.ast {
                        entry.ast_objects += ast.functions.len()
                            + ast.classes.len()
                            + ast.structs.len()
                            + ast.enums.len()
                            + ast.interfaces.len()
                            + ast.variables.len()
                            + ast.imports.len()
                            + ast.headings.len()
                            + ast.links.len()
                            + ast.code_blocks.len();
                    }
                } else {
                    other_stat.files += 1;
                    other_stat.lines += doc.lines_of_code.unwrap_or(0);
                    other_stat.size_bytes += doc.size_bytes;
                }
            }
        }

        // Convert sizes to human-readable
        for stat in lang_map.values_mut() {
            stat.size_human = SourceStats::format_size(stat.size_bytes);
        }
        other_stat.size_human = SourceStats::format_size(other_stat.size_bytes);

        let mut languages: Vec<LanguageStat> = lang_map.into_values().collect();
        languages.sort_by(|a, b| b.lines.cmp(&a.lines));

        // Add Other if it has files
        if other_stat.files > 0 {
            languages.push(other_stat);
        }

        let total_size_bytes = languages.iter().map(|l| l.size_bytes).sum();
        let total = LanguageStat {
            language: "Total".to_string(),
            files: languages.iter().map(|l| l.files).sum(),
            lines: languages.iter().map(|l| l.lines).sum(),
            ast_objects: languages.iter().map(|l| l.ast_objects).sum(),
            size_bytes: total_size_bytes,
            size_human: SourceStats::format_size(total_size_bytes),
        };

        Ok(SearchStats { languages, total })
    }

    /// Get all AST data for a source using scroll API.
    pub async fn get_all_ast(&self, index: &str) -> Result<Vec<SourceFileAnalysis>> {
        let mut all_results = Vec::new();

        if !self.client.index_exists(index).await? {
            return Ok(all_results);
        }

        let url = format!("{}/{}/_search?scroll=1m", self.client.base_url(), index);
        let query = json!({
            "size": 1000,
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

        loop {
            let hits = match resp["hits"]["hits"].as_array() {
                Some(h) => h,
                None => break,
            };
            if hits.is_empty() {
                break;
            }
            for h in hits {
                if let Ok(doc) = serde_json::from_value::<SourceFileAnalysis>(h["_source"].clone())
                {
                    all_results.push(doc);
                }
            }
            let scroll_id = match resp["_scroll_id"].as_str() {
                Some(id) => id,
                None => break,
            };
            resp = self
                .client
                .post(&format!("{}/_search/scroll", self.client.base_url()))
                .json(&json!({ "scroll": "1m", "scroll_id": scroll_id }))
                .send()
                .await?
                .json()
                .await?;
        }

        Ok(all_results)
    }

    /// Generate directory tree structure from file paths.
    pub async fn get_tree(&self, index: &str, level: usize) -> Result<SearchTree> {
        if !self.client.index_exists(index).await? {
            return Ok(SearchTree {
                tree: String::new(),
                directories: 0,
                files: 0,
            });
        }

        let hashes = self.get_hashes(index).await?;
        if hashes.is_empty() {
            return Ok(SearchTree {
                tree: String::new(),
                directories: 0,
                files: 0,
            });
        }

        // Extract root from first path
        let first_path = &hashes[0].path;
        let root = first_path
            .rsplit_once('/')
            .map(|(r, _)| r)
            .unwrap_or(first_path);
        let root_name = root.split('/').last().unwrap_or("project");

        // Extract relative paths
        let paths: Vec<String> = hashes
            .iter()
            .map(|h| {
                h.path
                    .strip_prefix(&format!("{}/", root))
                    .unwrap_or(&h.path)
                    .to_string()
            })
            .collect();

        // Build tree
        let (tree_string, dirs, files) = crate::utils::tree::build_tree(&paths, level);

        Ok(SearchTree {
            tree: format!("{}\n{}", root_name, tree_string.trim_end()),
            directories: dirs,
            files: files,
        })
    }

    /// Search files by path pattern in a specific source index.
    pub async fn get_files(
        &self,
        pattern: &str,
        index: &str,
        limit: usize,
    ) -> Result<Vec<SearchFile>> {
        // Check if index exists
        if !self.client.index_exists(index).await? {
            return Ok(Vec::new());
        }

        let query = json!({
            "size": limit,
            "_source": ["Root", "Path", "Content"],
            "query": {
                "wildcard": {
                    "Path": {
                        "value": format!("*{}*", pattern)
                    }
                }
            },
            "sort": [
                {
                    "_script": {
                        "type": "number",
                        "script": {
                            "source": "doc['Path'].value.length()"
                        },
                        "order": "asc"
                    }
                },
                {
                    "Path": {
                        "order": "asc"
                    }
                }
            ]
        });

        let resp = self.client.search(index, query).await?;
        let hits = resp["hits"]["hits"].as_array().unwrap();

        let results: Vec<SearchFile> = hits
            .iter()
            .filter_map(|h| serde_json::from_value(h["_source"].clone()).ok())
            .collect();

        Ok(results)
    }

    /// Search in project metadata (root path, license, summary).
    pub async fn search_meta(&self, pattern: &str, limit: usize) -> Result<Vec<SearchMeta>> {
        let index = AppConfig::meta_index();

        let query = json!({
            "size": limit,
            "query": {
                "bool": {
                    "should": [
                        { "wildcard": { "Root": { "value": format!("*{}*", pattern), "boost": 10.0 } } },
                        { "match": { "License.Name": { "query": pattern, "boost": 5.0 } } },
                        { "term": { "License.SpdxId": { "value": pattern, "boost": 5.0 } } },
                        { "match": { "Summary": { "query": pattern, "boost": 2.0 } } },
                    ]
                }
            }
        });

        let resp = self.client.search(&index, query).await?;
        let hits = resp["hits"]["hits"].as_array().unwrap();

        let results: Vec<SearchMeta> = hits
            .iter()
            .filter_map(|h| serde_json::from_value(h["_source"].clone()).ok())
            .collect();

        Ok(results)
    }

    /// Returns metadata for all indexed projects.
    pub async fn list_meta(&self) -> Result<Vec<SearchMeta>> {
        let index = AppConfig::meta_index();
        let query = json!({
            "size": 999,
            "query": { "match_all": {} }
        });
        let resp = self.client.search(&index, query).await?;
        let hits = resp["hits"]["hits"].as_array().unwrap();
        let results: Vec<SearchMeta> = hits
            .iter()
            .filter_map(|h| serde_json::from_value(h["_source"].clone()).ok())
            .collect();

        Ok(results)
    }

    /// Get all file hashes from a files index using scroll API.
    pub async fn get_hashes(&self, index: &str) -> Result<Vec<SearchFileHash>> {
        // Check if index exists
        if !self.client.index_exists(index).await? {
            return Ok(Vec::new());
        }

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
