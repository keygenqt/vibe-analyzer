//! list_projects tool handler.
//! Returns all indexed projects with minimal metadata.

use std::path::Path;
use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;
use serde_json::json;

use crate::mcp::server::AppState;
use crate::mcp::tools::data::responses::list_projects::ListProjectsResponse;
use crate::mcp::tools::data::responses::list_projects::ProjectInfo;

/// Tool for listing all knowledge sources.
#[derive(Clone)]
pub struct ToolListProjects {
    state: Arc<AppState>,
}

impl ToolListProjects {
    /// Creates a new ToolListProjects instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Returns list of available projects.
    pub async fn execute(&self) -> Result<CallToolResult, ErrorData> {
        let results = self.state.search_client.list_meta().await.map_err(|e| {
            ErrorData::internal_error(e.to_string(), Some(json!({ "detail": e.to_string() })))
        })?;

        let projects: Vec<ProjectInfo> = results
            .iter()
            .map(|m| {
                let name = Path::new(&m.root)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&m.root)
                    .to_string();

                ProjectInfo {
                    path: m.root.clone(),
                    name,
                    summary: m.summary.clone(),
                }
            })
            .collect();

        let total = projects.len();
        let response = ListProjectsResponse { projects, total };

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
