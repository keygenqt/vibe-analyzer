//! File-level analysis data structure.
//! Contains per-file metadata, AST data, and content statistics for a scanned file.

use serde::Deserialize;
use serde::Serialize;

use crate::scanner::ast::data::ast_data::AstData;

/// Analysis results for a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "FileAnalysis")]
pub struct ScanFileAnalysisOutput {
    /// Relative path from source root
    pub path: String,

    /// File extension without dot (e.g., "rs", "toml")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,

    /// Whether the file is text-based
    pub is_text: bool,

    /// File size in bytes
    pub size_bytes: u64,

    /// Number of lines (text files only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_of_code: Option<usize>,

    /// Number of characters (text files only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_chars: Option<usize>,

    /// AST data (if text and parsable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ast: Option<AstData>,

    /// Unique file identifier (BLAKE3 hash)
    pub hash: String,
}
