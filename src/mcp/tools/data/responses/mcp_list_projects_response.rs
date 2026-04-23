//! Response structure for the list_projects tool.

use serde::Deserialize;
use serde::Serialize;

/// Response from list_projects tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "ListProjects")]
pub struct McpListProjectsResponse {
    /// Available projects.
    pub projects: Vec<McpProjectInfoResponse>,
    /// Total count.
    pub total: usize,
}

/// Project information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "ProjectInfo")]
pub struct McpProjectInfoResponse {
    /// Full path.
    pub path: String,
    /// Project name.
    pub name: String,
    /// Brief description.
    pub summary: String,
}

impl McpListProjectsResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        format!(
            "[{} project(s).]\n\nUse this data to respond to the user.\n{}",
            self.projects.len(),
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
