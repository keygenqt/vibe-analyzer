//! Response structure for the show_stats tool.

use serde::Deserialize;
use serde::Serialize;

use crate::opensearch::data::responses::opensearch_stats_response::OpensearchStatsLanguageResponse;

/// Response from show_stats tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Stats")]
pub struct McpShowStatsResponse {
    /// Target project (omitted = all).
    pub target: Option<String>,
    /// Stats per language.
    pub languages: Vec<McpShowStatLanguageResponse>,
    /// Total across all languages.
    pub total: McpShowStatTotalResponse,
}

/// Statistics for a single language.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "StatLanguage")]
pub struct McpShowStatLanguageResponse {
    /// Language name.
    pub language: String,
    /// File count.
    pub files: usize,
    /// Total lines of code.
    pub lines: usize,
    /// AST objects count.
    pub ast_objects: usize,
    /// Human-readable size.
    pub size_human: String,
}

impl From<OpensearchStatsLanguageResponse> for McpShowStatLanguageResponse {
    fn from(l: OpensearchStatsLanguageResponse) -> Self {
        Self {
            language: l.language,
            files: l.files,
            lines: l.lines,
            ast_objects: l.ast_objects,
            size_human: l.size_human,
        }
    }
}

/// Total statistics across all languages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "StatTotal")]
pub struct McpShowStatTotalResponse {
    /// Total files.
    pub files: usize,
    /// Total lines.
    pub lines: usize,
    /// Total AST objects.
    pub ast_objects: usize,
    /// Total size.
    pub size_human: String,
}

impl From<OpensearchStatsLanguageResponse> for McpShowStatTotalResponse {
    fn from(t: OpensearchStatsLanguageResponse) -> Self {
        Self {
            files: t.files,
            lines: t.lines,
            ast_objects: t.ast_objects,
            size_human: t.size_human,
        }
    }
}

impl McpShowStatsResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        format!(
            "[{} language(s), {} file(s), {} line(s)]\n\nUse this data to respond to the user.\n{}",
            self.languages.len(),
            self.total.files,
            self.total.lines,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
