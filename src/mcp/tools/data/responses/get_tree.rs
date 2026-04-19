//! Response structure for the get_tree tool.

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTreeResponse {
    pub target: String,
    pub tree: String,
    pub total_files: usize,
    pub total_dirs: usize,
}

impl GetTreeResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        let json = serde_json::to_string_pretty(self).unwrap_or_default();
        format!(
            "[TOOL RESULT - Directory tree for '{}']\n{}\n\nUse this data to respond to the user.",
            self.target, json
        )
    }
}
