//! Export configuration.
//! Default format and output directory for scan exports.

use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::export::options::ExportFormat;
use crate::export::options::ExportType;

/// Export settings for scan commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Default export format (json, json5, toml, toon, xml, yaml).
    pub format: String,
    /// Default output directory.
    pub output_dir: PathBuf,
    /// File types to include in export.
    pub types: ExportTypesConfig,
    /// AST elements to include in export.
    pub include: ExportIncludeConfig,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: ExportFormat::Json.extension().to_string(),
            output_dir: dirs::download_dir().expect("Failed to get downloads directory"),
            types: ExportTypesConfig::default(),
            include: ExportIncludeConfig::default(),
        }
    }
}

/// File types to include in export output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTypesConfig {
    /// Include code files (Rust, Python, JS, etc.).
    pub code: bool,
    /// Include markdown files.
    pub markdown: bool,
    /// Include text files (configs, toml, yaml, etc.).
    pub text: bool,
    /// Include binary files (images, etc.).
    pub binary: bool,
}

impl Default for ExportTypesConfig {
    fn default() -> Self {
        Self {
            code: true,
            markdown: true,
            text: true,
            binary: true,
        }
    }
}

impl ExportTypesConfig {
    /// Returns list of enabled export file types.
    pub fn list(&self) -> Vec<ExportType> {
        let mut types = vec![];
        if self.code {
            types.push(ExportType::Code);
        }
        if self.markdown {
            types.push(ExportType::Markdown);
        }
        if self.text {
            types.push(ExportType::Text);
        }
        if self.binary {
            types.push(ExportType::Binary);
        }
        types
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
    /// Include markers from static analysis (TODO, FIXME, HACK, etc.).
    pub markers: bool,
    /// Include warnings from static analysis (unwrap, panic, bare except, etc.).
    pub warnings: bool,
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
            markers: false,
            warnings: false,
            body: false,
        }
    }
}
