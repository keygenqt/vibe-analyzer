use serde::{Deserialize, Serialize};

use crate::scanner::ast::parser::AstData;

/// Represents analysis results for a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysis {
    /// Relative path from project root
    pub path: String,
    /// File extension (e.g., "rs", "toml", "md") without the dot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    /// File size in bytes
    pub size_bytes: u64,
    /// Whether the file is text-based (true) or binary (false)
    pub is_text: bool,
    /// Number of lines in the file (only for text files)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_of_code: Option<usize>,
    /// Number of characters in the file (only for text files)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_chars: Option<usize>,
    /// AST data extracted from the file (if text and parsable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ast: Option<AstData>,
    /// Unique file identifier (BLAKE3) for content linking
    pub file_id: String,
}
