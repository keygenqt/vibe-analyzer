//! Response structure for the list_projects tool.

use serde::Deserialize;
use serde::Serialize;

/// Response from list_projects tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListProjectsResponse {
    /// List of available projects.
    pub projects: Vec<ProjectInfo>,
    /// Total number of projects.
    pub total: usize,
}

/// Information about a single project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    /// Unique identifier - full path to the project.
    pub path: String,
    /// Human-readable name extracted from the path.
    pub name: String,
    /// Brief description of the project.
    pub summary: String,
}

impl ListProjectsResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        let json = serde_json::to_string_pretty(self).unwrap_or_default();
        format!(
            "[TOOL RESULT - Project list ({} projects)]\n{}\n\nUse this data to respond to the user.",
            self.total, json
        )
    }
}
