//! Export configuration.
//! Default format and output directory for scan exports.

use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::export::options::ExportFormat;

/// Export settings for scan commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Default export format (json, json5, toml, toon, xml, yaml).
    pub format: String,
    /// Default output directory.
    pub output_dir: PathBuf,
    /// AST elements to include in export.
    pub include: ExportIncludeConfig,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: ExportFormat::Json.extension().to_string(),
            output_dir: dirs::download_dir().expect("Failed to get downloads directory"),
            include: ExportIncludeConfig::default(),
        }
    }
}

/// AST elements to include in export output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportIncludeConfig {
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
    /// Include body content (functions, classes, headings, etc.).
    pub body: bool,
}

impl Default for ExportIncludeConfig {
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
            body: false,
        }
    }
}
