//! Response structure for the get_file_content tool.

use serde::Deserialize;
use serde::Serialize;

/// Response from get_file_content tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "FileContent")]
pub struct McpGetFileContentResponse {
    /// Project root path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
    /// File path.
    pub path: String,
    /// Programming language of the file.
    pub language: String,
    /// Full file content.
    pub content: String,
}

impl McpGetFileContentResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        format!(
            "[File: {}]\n\nUse this data to respond to the user.\n{}",
            self.path,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
