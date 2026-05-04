//! Project statistics query.

use std::collections::HashMap;

use anyhow::Result;

use crate::{
    models::file_models::FileModels, opensearch::tools::tool_client::OpenSearchToolClient,
};

/// Statistics output.
pub struct ShowStatsOutput {
    pub languages: Vec<LanguageStatsOutput>,
    pub total: LanguageStatsOutput,
}

/// Per-language statistics output.
pub struct LanguageStatsOutput {
    pub language: String,
    pub files: usize,
    pub lines: usize,
    pub ast_objects: usize,
    pub size_bytes: u64,
    pub size_human: String,
}

impl OpenSearchToolClient {
    /// Get project statistics with per-language breakdown.
    pub async fn show_stats_search(&self) -> Result<ShowStatsOutput> {
        let docs = self.scroll_files_meta().await?;

        let mut lang_map: HashMap<String, LanguageStatsOutput> = HashMap::new();
        let mut other = LanguageStatsOutput {
            language: "Other".to_string(),
            files: 0,
            lines: 0,
            ast_objects: 0,
            size_bytes: 0,
            size_human: String::new(),
        };

        for doc in &docs {
            match doc {
                FileModels::Language(m) => {
                    let lang = &m.language;
                    let entry = lang_map.entry(lang.clone()).or_insert(LanguageStatsOutput {
                        language: lang.clone(),
                        files: 0,
                        lines: 0,
                        ast_objects: 0,
                        size_bytes: 0,
                        size_human: String::new(),
                    });
                    entry.files += 1;
                    entry.lines += m.lines_of_code.unwrap_or(0);
                    entry.size_bytes += m.size_bytes.unwrap_or(0);
                    entry.ast_objects += m.functions.len()
                        + m.classes.len()
                        + m.structs.len()
                        + m.enums.len()
                        + m.interfaces.len()
                        + m.variables.len()
                        + m.imports.len()
                        + m.headings.len()
                        + m.links.len()
                        + m.code_blocks.len();
                }
                FileModels::Text(m) => {
                    other.files += 1;
                    other.size_bytes += m.size_bytes.unwrap_or(0);
                }
                FileModels::Binary(m) => {
                    other.files += 1;
                    other.size_bytes += m.size_bytes.unwrap_or(0);
                }
            }
        }

        // Format sizes
        for stat in lang_map.values_mut() {
            stat.size_human = Self::format_size(stat.size_bytes);
        }
        other.size_human = Self::format_size(other.size_bytes);

        let mut languages: Vec<LanguageStatsOutput> = lang_map.into_values().collect();
        languages.sort_by(|a, b| b.lines.cmp(&a.lines));

        if other.files > 0 {
            languages.push(other);
        }

        let total_size_bytes: u64 = languages.iter().map(|l| l.size_bytes).sum();
        let total = LanguageStatsOutput {
            language: "Total".to_string(),
            files: languages.iter().map(|l| l.files).sum(),
            lines: languages.iter().map(|l| l.lines).sum(),
            ast_objects: languages.iter().map(|l| l.ast_objects).sum(),
            size_bytes: total_size_bytes,
            size_human: Self::format_size(total_size_bytes),
        };

        Ok(ShowStatsOutput { languages, total })
    }

    /// Format byte size to human-readable string.
    fn format_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit = 0;
        while size >= 1024.0 && unit < UNITS.len() - 1 {
            size /= 1024.0;
            unit += 1;
        }
        if unit == 0 {
            format!("{} {}", bytes, UNITS[unit])
        } else {
            format!("{:.1} {}", size, UNITS[unit])
        }
    }
}
