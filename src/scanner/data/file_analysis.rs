use serde::{Deserialize, Serialize};

use crate::scanner::ast::parser::AstData;

/// Represents analysis results for a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysis {
    /// Relative path from project root
    pub path: String,
    /// MIME type of the file (e.g., "text/x-rust", "text/xml")
    pub mime_type: String,
    /// Detected programming language (e.g., "rs", "py", "js")
    pub language: Option<String>,
    /// File size in bytes
    pub size_bytes: u64,
    /// BLAKE3 hash of file content
    pub content_hash: String,
    /// Whether the file is text-based (true) or binary (false)
    pub is_text: bool,
    /// AST data extracted from the file (if text and parsable)
    pub ast: Option<AstData>,
}
