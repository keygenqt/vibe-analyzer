//! Response structure for the show_tree tool.

use std::convert::TryFrom;

use anyhow::Result;
use serde::Serialize;

use crate::mcp::responses::McpResponse;
use crate::opensearch::tools::query::show_tree_search::ShowTreeOutput;

/// Response from show_tree tool.
#[derive(Debug, Serialize)]
pub struct ShowTreeResponse {
    /// Directory tree as text.
    pub tree: String,
    /// Number of files.
    pub files: usize,
    /// Number of directories.
    pub directories: usize,
}

/// Converts tree output to MCP response.
impl TryFrom<ShowTreeOutput> for ShowTreeResponse {
    type Error = anyhow::Error;

    fn try_from(output: ShowTreeOutput) -> Result<Self> {
        Ok(ShowTreeResponse {
            tree: output.tree,
            files: output.files,
            directories: output.directories,
        })
    }
}

impl McpResponse for ShowTreeResponse {
    /// Returns a formatted tool result with JSON data.
    fn format(&self) -> String {
        format!(
            "[{} file(s), {} dir(s)]\nUse this data to respond to the user.\n\n{}",
            self.files,
            self.directories,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
