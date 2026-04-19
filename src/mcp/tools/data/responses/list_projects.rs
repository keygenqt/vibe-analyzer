//! Response structure for the list_projects tool.

use serde::{Deserialize, Serialize};

/// Response from list_projects tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListProjectsResponse {
    /// List of available projects.
    pub projects: Vec<ProjectInfo>,
    /// Total number of projects.
    pub total: usize,
}

/// Information about a single project.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProjectInfo {
    /// Unique identifier - full path to the project.
    pub path: String,
    /// Human-readable name extracted from the path.
    pub name: String,
    /// Brief description of the project.
    pub summary: String,
}
