//! Export options and configuration for controlling output behavior.
//! Defines available formats and user-controllable export settings.

use std::path::PathBuf;

use clap::ValueEnum;

/// Available export formats supported by the analyzer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum ExportFormat {
    /// Compact JSON format with minimal size (no extra whitespace).
    Json,
    /// Human-readable JSON5 format with comments and trailing commas.
    Json5,
    /// Token-efficient TOON format optimized for LLM consumption.
    Toon,
    /// Verbose XML format for enterprise and legacy system integration.
    Xml,
    /// Human-readable YAML format.
    Yaml,
    /// TOML configuration format.
    Toml,
}

impl ExportFormat {
    /// Returns the file extension for this format (without the dot).
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Json => "json",
            ExportFormat::Json5 => "json5",
            ExportFormat::Toon => "toon",
            ExportFormat::Xml => "xml",
            ExportFormat::Yaml => "yaml",
            ExportFormat::Toml => "toml",
        }
    }
}

/// User-controllable options for the export operation.
#[derive(Debug, Clone)]
pub struct ExportOptions {
    /// File name prefix for default output path.
    pub prefix: String,

    /// Output format (JSON, JSON5, TOON, XML, YAML, TOML).
    pub format: ExportFormat,

    /// Custom output path. If None, uses default Downloads directory.
    pub output: Option<PathBuf>,
}

impl ExportOptions {
    /// Creates a new ExportOptions instance with the specified format and output path.
    pub fn new(prefix: &str, format: ExportFormat, output: Option<PathBuf>) -> Self {
        Self {
            prefix: prefix.to_string(),
            format,
            output,
        }
    }
}
