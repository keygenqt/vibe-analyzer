//! Response structure for the show_structure tool.

use serde::Deserialize;
use serde::Serialize;

/// Response from show_structure tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Structure")]
pub struct McpShowStructureResponse {
    /// Target project (omitted = all).
    pub target: Option<String>,
    /// Directory tree as text.
    pub tree: String,
    /// Total files.
    pub total_files: usize,
    /// Total directories.
    pub total_dirs: usize,
}

impl McpShowStructureResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        let target = self.target.as_deref().unwrap_or("all projects");
        format!(
            "[{} file(s), {} dir(s) in {}]\n\nUse this data to respond to the user.\n{}",
            self.total_files,
            self.total_dirs,
            target,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
