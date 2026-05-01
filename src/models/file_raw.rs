//! Raw file model.
//! Contains full file content for storage and retrieval.

use serde::{Deserialize, Serialize};

use crate::models::file_trait::FileModel;

/// Raw file with full content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRaw {
    /// Relative path from project root.
    pub path: String,
    /// File extension without dot.
    pub extension: Option<String>,
    /// Full file content as string.
    pub content: String,
}

impl FileModel for FileRaw {
    fn path(&self) -> &str {
        &self.path
    }
}
