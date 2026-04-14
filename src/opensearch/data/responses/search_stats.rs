//! Statistics response structure for project overview.
//! Contains per-language breakdown and totals.

use serde::Deserialize;
use serde::Serialize;

/// Per-language statistics.
#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageStat {
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

/// Project statistics response.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchStats {
    /// Per-language breakdown.
    pub languages: Vec<LanguageStat>,
    /// Total across all languages.
    pub total: LanguageStat,
}
