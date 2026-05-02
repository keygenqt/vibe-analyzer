//! Raw file model.
//! Contains full file content for storage and retrieval.

use serde::{Deserialize, Serialize};

use crate::models::{file_model::FileModel, file_models::FileModels};

/// Raw file with full content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTextModel {
    /// Absolute path to file.
    pub path: String,
    /// First N bytes of file content for preview.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub preview: Option<String>,
    /// Optional file extension without dot.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub extension: Option<String>,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub size_bytes: Option<u64>,
    /// BLAKE3 hash.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hash: Option<String>,
    /// Full file content.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content: Option<String>,
}

impl FileModel for FileTextModel {
    fn path(&self) -> &str {
        &self.path
    }

    fn content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    fn format_ast(&self) -> FileModels {
        FileModels::Text(FileTextModel {
            path: self.path.clone(),
            preview: self.preview.clone().or_else(|| self.preview(None)),
            extension: self.extension.clone().or_else(|| self.extension()),
            size_bytes: self.size_bytes.or_else(|| self.size_bytes()),
            hash: None,
            content: None,
        })
    }

    fn format_analyze(&self) -> FileModels {
        self.format_ast()
    }
}
