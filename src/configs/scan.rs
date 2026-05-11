//! Scan configuration.
//! Controls which AST elements are extracted from source code.

use serde::Deserialize;
use serde::Serialize;

/// AST element filters for code scanning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    /// Include import statements.
    pub imports: bool,
    /// Include function signatures.
    pub functions: bool,
    /// Include variable declarations.
    pub variables: bool,
    /// Include enum declarations.
    pub enums: bool,
    /// Include interface and trait declarations.
    pub interfaces: bool,
    /// Include class declarations.
    pub classes: bool,
    /// Include struct declarations.
    pub structs: bool,
    /// Include doc comments on elements.
    pub comments: bool,
    /// Include file header comments.
    pub header_comments: bool,
    /// Include markdown headings.
    pub headings: bool,
    /// Include markdown links.
    pub links: bool,
    /// Include code blocks.
    pub code_blocks: bool,
}

impl Default for ScanConfig {
    /// Scan everything except imports by default.
    fn default() -> Self {
        Self {
            imports: true,
            functions: true,
            variables: true,
            enums: true,
            interfaces: true,
            classes: true,
            structs: true,
            comments: true,
            header_comments: true,
            headings: true,
            links: true,
            code_blocks: true,
        }
    }
}
