//! Response structure for the stats tool.

use serde::Deserialize;
use serde::Serialize;

/// Response from stats tool.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Stats")]
pub struct McpStatsResponse {
    /// Target project (omitted = all).
    pub target: Option<String>,
    /// Stats per language.
    pub languages: Vec<McpStatLanguageResponse>,
    /// Total across all languages.
    pub total: McpStatTotalResponse,
}

/// Statistics for a single language.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "StatLanguage")]
pub struct McpStatLanguageResponse {
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

/// Total statistics across all languages.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "StatTotal")]
pub struct McpStatTotalResponse {
    /// Total files.
    pub files: usize,
    /// Total lines.
    pub lines: usize,
    /// Total AST objects.
    pub ast_objects: usize,
    /// Total size.
    pub size_human: String,
}

impl McpStatsResponse {
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
