//! show_projects tool handler.
//! Returns all indexed projects with minimal metadata.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::mcp::server::AppState;
use crate::mcp::tools::data::responses::mcp_show_projects_response::McpShowProjectInfoResponse;
use crate::mcp::tools::data::responses::mcp_show_projects_response::McpShowProjectsResponse;

/// Tool for listing all knowledge sources.
#[derive(Clone)]
pub struct ShowProjectsTool {
    state: Arc<AppState>,
}

impl ShowProjectsTool {
    /// Creates a new ShowProjectsTool instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Returns list of available projects.
    pub async fn execute(&self) -> Result<CallToolResult, ErrorData> {
        // Fetch and map projects from OpenSearch
        let projects: Vec<McpShowProjectInfoResponse> = self
            .state
            .search_client
            .show_projects_search()
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?
            .iter()
            .map(|m| m.into())
            .collect();

        // Build response
        Ok(CallToolResult::success(vec![Content::text(
            McpShowProjectsResponse {
                total: projects.len(),
                projects,
            }
            .format(),
        )]))
    }
}
