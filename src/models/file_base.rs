//! Base file information model.
//! Core fields: path, content, hash, size, and line count.

use serde::Deserialize;
use serde::Serialize;

use crate::models::trait_base_model::TraitBaseModel;
use crate::models::trait_format_model::TraitFormatModel;

/// Base file information — path, content, hash, size, and line count.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileBaseModel {
    /// Absolute path to file.
    pub path: String,
    /// BLAKE3 hash of file content.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hash: Option<String>,
    /// Full file content.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content: Option<String>,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub size_bytes: Option<u64>,
    /// Number of lines in file.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub size_lines: Option<usize>,
}

/// Base file model implementation.
impl TraitBaseModel for FileBaseModel {
    fn path(&self) -> &str {
        &self.path
    }

    fn content(&self) -> Option<&str> {
        self.content.as_deref()
    }
}

/// Format implementation for base file model.
impl TraitFormatModel for FileBaseModel {
    fn format_ast(&self) -> Self {
        Self {
            path: self.path.clone(),
            size_bytes: self.size_bytes.or_else(|| self.size_bytes()),
            size_lines: self.size_lines.or_else(|| self.size_lines()),
            hash: None,
            content: None,
        }
    }

    fn format_analyze(&self) -> Self {
        self.format_ast()
    }

    fn format_index(&self) -> Self {
        self.format_ast()
    }

    fn format_tool(&self) -> Self {
        Self {
            path: self.path.clone(),
            size_bytes: self.size_bytes.or_else(|| self.size_bytes()),
            size_lines: None,
            hash: None,
            content: None,
        }
    }
}
