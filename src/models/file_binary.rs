//! Binary file model.
//! Represents a non-text file (images, archives, executables).

use serde::{Deserialize, Serialize};

use crate::models::{file_model::FileModel, file_models::FileModels};

/// Binary file detected by content analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileBinaryModel {
    /// Absolute path to file.
    pub path: String,
    /// Optional file extension without dot.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub extension: Option<String>,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub size_bytes: Option<u64>,
    /// BLAKE3 hash.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hash: Option<String>,
}

impl FileModel for FileBinaryModel {
    fn path(&self) -> &str {
        &self.path
    }

    fn format_ast(&self) -> FileModels {
        FileModels::Binary(FileBinaryModel {
            path: self.path.clone(),
            extension: self.extension.clone().or_else(|| self.extension()),
            size_bytes: self.size_bytes.or_else(|| self.size_bytes()),
            hash: None,
        })
    }

    fn format_analyze(&self) -> FileModels {
        self.format_ast()
    }

    fn format_index(&self) -> FileModels {
        self.format_ast()
    }
}
