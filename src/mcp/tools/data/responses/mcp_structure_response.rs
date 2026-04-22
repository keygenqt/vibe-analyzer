//! Response data for the structure tool.

use serde::Deserialize;
use serde::Serialize;

/// Response from structure tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Structure")]
pub struct McpStructureResponse {
    /// Project path.
    pub target: String,
    /// Directory tree as text.
    pub tree: String,
    /// Total files.
    pub total_files: usize,
    /// Total directories.
    pub total_dirs: usize,
}

impl McpStructureResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        let json = serde_json::to_string(self).unwrap_or_default();
        format!(
            "[TOOL RESULT - Directory tree for '{}']\n{}\n\nUse this data to respond to the user.",
            self.target, json
        )
    }
}
