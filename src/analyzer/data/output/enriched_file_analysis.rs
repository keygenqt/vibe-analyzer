//! Enriched file analysis structure.
//! Extends scanner FileAnalysis with LLM-generated metadata.

use crate::scanner::ast::data::ast_data::AstData;
use serde::Deserialize;
use serde::Serialize;

/// File analysis enriched with LLM-generated description and search tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnrichedFileAnalysis {
    /// LLM-generated description of file purpose and main functionality
    pub description: Option<String>,
    /// Search tags for improved discoverability
    pub tags: Option<Vec<String>>,
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
    pub file_id: String,
}
