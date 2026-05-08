//! Response structure for the get_file_content tool.

use serde::Serialize;

use crate::mcp::responses::McpResponse;

/// Response from get_file_content tool.
#[derive(Debug, Serialize)]
pub struct GetFileContentResponse {
    /// File path.
    pub path: String,
    /// Full file content.
    pub content: String,
}

impl McpResponse for GetFileContentResponse {
    /// Returns a formatted tool result with JSON data.
    fn format(&self) -> String {
        format!(
            "[File: {}]\nUse this data to respond to the user.\n\n{}",
            self.path,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
