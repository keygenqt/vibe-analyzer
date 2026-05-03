//! Search functionality for OpenSearch indices.
//! Provides meta, ast, file, and get queries.

use std::collections::HashMap;

use anyhow::Result;
use serde_json::Value;
use serde_json::json;

use crate::opensearch::tools::tools_client::OpenSearchToolsClient;

impl OpenSearchToolsClient {
    /// Get statistics for multiple sources.
    pub async fn show_stats_search(&self, indexes: &[String]) -> Result<OpensearchStatsResponse> {
        let mut lang_map: HashMap<String, OpensearchStatsLanguageResponse> = HashMap::new();
        let mut other_stat = OpensearchStatsLanguageResponse {
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
                    let entry = lang_map.entry(lang_name.to_string()).or_insert(
                        OpensearchStatsLanguageResponse {
                            language: lang_name.to_string(),
                            files: 0,
                            lines: 0,
                            ast_objects: 0,
                            size_bytes: 0,
                            size_human: String::new(),
                        },
                    );

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
            stat.size_human = ScanProjectStatsOutput::format_size(stat.size_bytes);
        }
        other_stat.size_human = ScanProjectStatsOutput::format_size(other_stat.size_bytes);

        let mut languages: Vec<OpensearchStatsLanguageResponse> = lang_map.into_values().collect();
        languages.sort_by(|a, b| b.lines.cmp(&a.lines));

        // Add Other if it has files
        if other_stat.files > 0 {
            languages.push(other_stat);
        }

        let total_size_bytes = languages.iter().map(|l| l.size_bytes).sum();
        let total = OpensearchStatsLanguageResponse {
            language: "Total".to_string(),
            files: languages.iter().map(|l| l.files).sum(),
            lines: languages.iter().map(|l| l.lines).sum(),
            ast_objects: languages.iter().map(|l| l.ast_objects).sum(),
            size_bytes: total_size_bytes,
            size_human: ScanProjectStatsOutput::format_size(total_size_bytes),
        };

        Ok(OpensearchStatsResponse { languages, total })
    }

    /// Get all AST data for a source using scroll API.
    pub async fn get_all_ast(&self, index: &str) -> Result<Vec<OpensearchFileStatsRequest>> {
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
                if let Ok(doc) =
                    serde_json::from_value::<OpensearchFileStatsRequest>(h["_source"].clone())
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
}
