//! Response structure for the show_projects tool.

use std::convert::TryFrom;

use anyhow::Result;
use serde::Serialize;

use crate::{mcp::responses::McpResponse, models::project::ProjectModel};

/// Response from show_projects tool.
#[derive(Debug, Serialize)]
pub struct ShowProjectsResponse {
    /// Available projects.
    pub projects: Vec<ShowProjectInfoResponse>,
    /// Total count.
    pub total: usize,
}

/// Project information.
#[derive(Debug, Serialize)]
pub struct ShowProjectInfoResponse {
    /// Full path.
    pub path: String,
    /// Project name.
    pub name: String,
    /// Brief description.
    pub summary: String,
}

/// Converts project model to summary info.
impl TryFrom<&ProjectModel> for ShowProjectInfoResponse {
    type Error = anyhow::Error;

    fn try_from(project: &ProjectModel) -> Result<Self> {
        let name = project
            .path
            .rsplit('/')
            .next()
            .unwrap_or(&project.path)
            .to_string();
        Ok(ShowProjectInfoResponse {
            path: project.path.clone(),
            name,
            summary: project.summary.clone().unwrap_or_default(),
        })
    }
}

/// Converts slice of project models to show projects response.
impl TryFrom<&[ProjectModel]> for ShowProjectsResponse {
    type Error = anyhow::Error;

    fn try_from(projects: &[ProjectModel]) -> Result<Self> {
        let infos: Vec<ShowProjectInfoResponse> = projects
            .iter()
            .map(ShowProjectInfoResponse::try_from)
            .collect::<Result<Vec<_>>>()?;
        Ok(ShowProjectsResponse {
            total: infos.len(),
            projects: infos,
        })
    }
}

impl McpResponse for ShowProjectsResponse {
    fn format(&self) -> String {
        format!(
            "[{} project(s)]\nUse this data to respond to the user.\n\n{}",
            self.projects.len(),
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
