//! Binary file model.
//! Represents a non-text file (images, archives, executables).

use serde::{Deserialize, Serialize};

use crate::models::file_trait::FileModel;

/// Binary file detected by content analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileBinary {
    /// Relative path from project root.
    pub path: String,
    /// Optional file extension without dot.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub extension: Option<String>,
}

impl FileModel for FileBinary {
    fn path(&self) -> &str {
        &self.path
    }
}
