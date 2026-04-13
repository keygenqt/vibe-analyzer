//! Search result structure for file queries.
//! Contains root path, relative path, and raw file content.

use serde::Deserialize;
use serde::Serialize;

/// File data returned from file search and retrieval queries.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchFile {
    /// Absolute path to source root.
    pub root: String,
    /// Relative path from source root.
    pub path: String,
    /// Raw file content.
    pub content: String,
}
