//! show_projects tool handler.
//! Returns all indexed projects with minimal metadata.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::{
    mcp::{
        params::show_projects_params::ShowProjectsParams,
        responses::{McpResponse, show_projects_response::ShowProjectsResponse},
        server::AppState,
    },
    utils::mcp_normalize::number_normalize,
};

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
    pub async fn execute(&self, params: ShowProjectsParams) -> Result<CallToolResult, ErrorData> {
        let size = number_normalize(params.size, ShowProjectsParams::default_size(), Some(100));

        let client = self
            .state
            .open_search_client
            .client_tool_projects_meta()
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        let projects = client
            .show_projects_search(size)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        let response = ShowProjectsResponse::try_from(projects.as_slice())
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
