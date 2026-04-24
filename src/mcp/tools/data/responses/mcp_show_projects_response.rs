//! Response structure for the show_projects tool.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::opensearch::data::responses::opensearch_project_meta_response::OpensearchProjectMetaResponse;

/// Response from show_projects tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Projects")]
pub struct McpShowProjectsResponse {
    /// Available projects.
    pub projects: Vec<McpShowProjectInfoResponse>,
    /// Total count.
    pub total: usize,
}

/// Project information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "ProjectInfo")]
pub struct McpShowProjectInfoResponse {
    /// Full path.
    pub path: String,
    /// Project name.
    pub name: String,
    /// Brief description.
    pub summary: String,
}

impl From<&OpensearchProjectMetaResponse> for McpShowProjectInfoResponse {
    fn from(m: &OpensearchProjectMetaResponse) -> Self {
        Self {
            path: m.root.clone(),
            name: Path::new(&m.root)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&m.root)
                .to_string(),
            summary: m.summary.clone(),
        }
    }
}

impl McpShowProjectsResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        format!(
            "[{} project(s).]\n\nUse this data to respond to the user.\n{}",
            self.projects.len(),
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
