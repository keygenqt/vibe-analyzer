//! Response structure for the get_stats tool.

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetStatsResponse {
    /// Target project (None means all projects).
    pub target: Option<String>,
    /// Per-language statistics.
    pub languages: Vec<LanguageStat>,
    /// Total across all languages.
    pub total: TotalStat,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LanguageStat {
    pub language: String,
    pub files: usize,
    pub lines: usize,
    pub ast_objects: usize,
    pub size_human: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TotalStat {
    pub files: usize,
    pub lines: usize,
    pub ast_objects: usize,
    pub size_human: String,
}

impl GetStatsResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        let json = serde_json::to_string_pretty(self).unwrap_or_default();
        format!(
            "[TOOL RESULT - Statistics for {}]\n{}\n\nUse this data to respond to the user.",
            self.target.as_deref().unwrap_or("all projects"),
            json
        )
    }
}
