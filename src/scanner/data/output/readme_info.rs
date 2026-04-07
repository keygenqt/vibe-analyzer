//! README file information structure.
//! Stores path, content, and size of a detected README file.

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReadmeInfo {
    /// Path to README file
    pub path: String,
    /// Full content of README file
    pub content: String,
    /// File size in bytes
    pub size_bytes: u64,
}
