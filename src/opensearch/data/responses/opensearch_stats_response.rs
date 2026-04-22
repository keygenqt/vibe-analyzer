//! Statistics response structure for project overview.
//! Contains per-language breakdown and totals.

use serde::Deserialize;
use serde::Serialize;

/// Project statistics response.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Stats")]
pub struct OpensearchStatsResponse {
    /// Per-language breakdown.
    pub languages: Vec<OpensearchStatsLanguageResponse>,
    /// Total across all languages.
    pub total: OpensearchStatsLanguageResponse,
}

/// Per-language statistics.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "StatsLanguage")]
pub struct OpensearchStatsLanguageResponse {
    /// Language name.
    pub language: String,
    /// Number of files.
    pub files: usize,
    /// Total lines of code.
    pub lines: usize,
    /// Number of AST objects extracted.
    pub ast_objects: usize,
    /// Total size in bytes.
    #[serde(skip)]
    pub size_bytes: u64,
    /// Human readable size.
    pub size_human: String,
}
