//! Search functionality for OpenSearch indices.
//! Provides meta, ast, file, and get queries.

use std::collections::HashMap;

use anyhow::Result;
use serde_json::Value;
use serde_json::json;

use crate::configs::app::AppConfig;
use crate::opensearch::client::OpenSearchClient;
use crate::opensearch::data::requests::source_file_analysis::SourceFileAnalysis;
use crate::opensearch::data::responses::search_file_hash::SearchFileHash;
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

        // Find common root prefix of all paths
        let common_root = hashes
            .iter()
            .map(|h| h.path.as_str())
            .reduce(|a, b| {
                let common: String = a
                    .chars()
                    .zip(b.chars())
                    .take_while(|(x, y)| x == y)
                    .map(|(c, _)| c)
                    .collect();
                Box::leak(Box::new(common)) as &str
            })
            .unwrap_or("");

        // Trim to last '/' to get the project root directory
        let root = if let Some(last_slash) = common_root.rfind('/') {
            &common_root[..last_slash]
        } else {
            common_root
        };
        // Collect all paths
        let paths: Vec<String> = hashes.iter().map(|h| h.path.clone()).collect();
        // Build tree
        let (tree, dirs, files) = crate::utils::tree::build_tree(root, &paths, level);
        // Result
        Ok(SearchTree {
            tree,
            directories: dirs,
            files,
        })
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

    // /// @todo
    // /// Search in AST elements.
    // pub async fn search_ast(
    //     &self,
    //     indexes: &[String],
    //     pattern: &str,
    //     keywords: &[String],
    //     limit: usize,
    //     include_content: bool,
    // ) -> Result<Vec<SearchAst>> {
    //     // Combine original query with translations for multilingual search
    //     let mut all_terms = vec![pattern.to_string()];
    //     all_terms.extend(keywords.iter().take(5).cloned());
    //     let query_string = all_terms.join(" OR ");

    //     // Join indexes with comma for multi-index search
    //     let index_list = indexes.join(",");

    //     // Query to OpenSearch
    //     // Optimized OpenSearch query
    //     let query = json!({
    //         "size": limit,
    //         "timeout": "30s",
    //         "query": {
    //             "bool": {
    //                 "should": [
    //                     // Tags (combined)
    //                     {
    //                         "multi_match": {
    //                             "query": query_string,
    //                             "fields": [
    //                                 "tags^5.0",
    //                                 "tags.*^5.0",
    //                                 "ast.tags^4.0"
    //                             ],
    //                             "fuzziness": "AUTO"
    //                         }
    //                     },
    //                     // Headings (critical)
    //                     {
    //                         "nested": {
    //                             "path": "ast.headings",
    //                             "query": {
    //                                 "multi_match": {
    //                                     "query": query_string,
    //                                     "fields": [
    //                                         "ast.headings.title^4.0",
    //                                         "ast.headings.title.*^4.0"
    //                                     ],
    //                                     "fuzziness": "AUTO"
    //                                 }
    //                             }
    //                         }
    //                     },
    //                     // Summary + Description + README (combined)
    //                     {
    //                         "multi_match": {
    //                             "query": query_string,
    //                             "fields": [
    //                                 "summary^3.5",
    //                                 "summary.*^3.5",
    //                                 "description^3.5",
    //                                 "description.*^3.5",
    //                                 "readme.content^3.5",
    //                                 "readme.content.*^3.5"
    //                             ],
    //                             "fuzziness": "AUTO"
    //                         }
    //                     },
    //                     // Header comments
    //                     {
    //                         "multi_match": {
    //                             "query": query_string,
    //                             "fields": [
    //                                 "ast.header_comments^4.0",
    //                                 "ast.header_comments.*^4.0"
    //                             ],
    //                             "fuzziness": "AUTO"
    //                         }
    //                     },
    //                     // Frontmatter
    //                     {
    //                         "match": {
    //                             "ast.frontmatter": {
    //                                 "query": query_string,
    //                                 "boost": 4.0,
    //                                 "fuzziness": "AUTO"
    //                             }
    //                         }
    //                     },
    //                     // Functions
    //                     {
    //                         "nested": {
    //                             "path": "ast.functions",
    //                             "query": {
    //                                 "bool": {
    //                                     "should": [
    //                                         {
    //                                             "match": {
    //                                                 "ast.functions.signature": {
    //                                                     "query": query_string,
    //                                                     "boost": 4.0
    //                                                 }
    //                                             }
    //                                         },
    //                                         {
    //                                             "multi_match": {
    //                                                 "query": query_string,
    //                                                 "fields": [
    //                                                     "ast.functions.comments^3.5",
    //                                                     "ast.functions.comments.*^3.5"
    //                                                 ]
    //                                             }
    //                                         }
    //                                     ]
    //                                 }
    //                             }
    //                         }
    //                     },
    //                     // Classes
    //                     {
    //                         "nested": {
    //                             "path": "ast.classes",
    //                             "query": {
    //                                 "bool": {
    //                                     "should": [
    //                                         {
    //                                             "match": {
    //                                                 "ast.classes.signature": {
    //                                                     "query": query_string,
    //                                                     "boost": 4.0
    //                                                 }
    //                                             }
    //                                         },
    //                                         {
    //                                             "multi_match": {
    //                                                 "query": query_string,
    //                                                 "fields": [
    //                                                     "ast.classes.comments^3.5",
    //                                                     "ast.classes.comments.*^3.5"
    //                                                 ]
    //                                             }
    //                                         }
    //                                     ]
    //                                 }
    //                             }
    //                         }
    //                     },
    //                     // Structs
    //                     {
    //                         "nested": {
    //                             "path": "ast.structs",
    //                             "query": {
    //                                 "bool": {
    //                                     "should": [
    //                                         {
    //                                             "match": {
    //                                                 "ast.structs.signature": {
    //                                                     "query": query_string,
    //                                                     "boost": 4.0
    //                                                 }
    //                                             }
    //                                         },
    //                                         {
    //                                             "multi_match": {
    //                                                 "query": query_string,
    //                                                 "fields": [
    //                                                     "ast.structs.comments^3.5",
    //                                                     "ast.structs.comments.*^3.5"
    //                                                 ]
    //                                             }
    //                                         }
    //                                     ]
    //                                 }
    //                             }
    //                         }
    //                     },
    //                     // Enums
    //                     {
    //                         "nested": {
    //                             "path": "ast.enums",
    //                             "query": {
    //                                 "bool": {
    //                                     "should": [
    //                                         {
    //                                             "match": {
    //                                                 "ast.enums.signature": {
    //                                                     "query": query_string,
    //                                                     "boost": 4.0
    //                                                 }
    //                                             }
    //                                         },
    //                                         {
    //                                             "multi_match": {
    //                                                 "query": query_string,
    //                                                 "fields": [
    //                                                     "ast.enums.comments^3.5",
    //                                                     "ast.enums.comments.*^3.5"
    //                                                 ]
    //                                             }
    //                                         }
    //                                     ]
    //                                 }
    //                             }
    //                         }
    //                     },
    //                     // Interfaces (signature only, no comments)
    //                     {
    //                         "nested": {
    //                             "path": "ast.interfaces",
    //                             "query": {
    //                                 "match": {
    //                                     "ast.interfaces.signature": {
    //                                         "query": query_string,
    //                                         "boost": 4.0
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     },
    //                     // Imports
    //                     {
    //                         "match": {
    //                             "ast.imports": {
    //                                 "query": query_string,
    //                                 "boost": 3.5
    //                             }
    //                         }
    //                     },
    //                     // Links
    //                     {
    //                         "nested": {
    //                             "path": "ast.links",
    //                             "query": {
    //                                 "multi_match": {
    //                                     "query": query_string,
    //                                     "fields": [
    //                                         "ast.links.text^3.0",
    //                                         "ast.links.text.*^3.0"
    //                                     ],
    //                                     "fuzziness": "AUTO"
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 ],
    //                 "minimum_should_match": 1
    //             }
    //         },
    //     });

    //     let resp = self.client.search(&index_list, query).await?;
    //     let hits = resp["hits"]["hits"].as_array().unwrap();

    //     let mut results: Vec<SearchAst> = Vec::new();

    //     for h in hits {
    //         let analysis: SourceFileAnalysis = match serde_json::from_value(h["_source"].clone()) {
    //             Ok(a) => a,
    //             Err(_) => continue,
    //         };
    //         let ast = match analysis.ast.as_ref() {
    //             Some(a) => a,
    //             None => continue,
    //         };
    //         let mut search_ast = SearchAst {
    //             root: analysis.root.clone(),
    //             path: analysis.path.clone(),
    //             header_comments: ast.header_comments.clone(),
    //             imports: ast.imports.clone(),
    //             variables: ast.variables.clone(),
    //             functions: ast.functions.clone(),
    //             classes: ast.classes.clone(),
    //             structs: ast.structs.clone(),
    //             enums: ast.enums.clone(),
    //             interfaces: ast.interfaces.clone(),
    //             frontmatter: ast.frontmatter.clone(),
    //             headings: ast.headings.clone(),
    //             links: ast.links.clone(),
    //             code_blocks: ast.code_blocks.clone(),
    //             content: None,
    //         };

    //         // If include_content, fetch full file from files index
    //         if include_content {
    //             let root = PathBuf::from(&analysis.root);
    //             let files_index = AppConfig::files_index(&root);
    //             if let Ok(content) = self.get_file_content(&files_index, &analysis.path).await {
    //                 search_ast.content = Some(content);
    //             }
    //         }

    //         results.push(search_ast);
    //     }

    //     Ok(results)
    // }

    // /// @todo
    // /// Fetches full file content from files index.
    // async fn get_file_content(&self, index: &str, path: &str) -> Result<String> {
    //     let query = json!({
    //         "size": 1,
    //         "query": {
    //             "term": {
    //                 "path": path
    //             }
    //         }
    //     });
    //     let resp = self.client.search(index, query).await?;
    //     let hits = resp["hits"]["hits"].as_array().unwrap();
    //     if let Some(hit) = hits.first() {
    //         if let Some(content) = hit["_source"]["content"].as_str() {
    //             return Ok(content.to_string());
    //         }
    //     }
    //     anyhow::bail!("File not found: {}", path)
    // }
}
