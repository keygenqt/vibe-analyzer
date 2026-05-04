//! Tool for retrieving file content.
//! Handles full file content requests.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::{
    mcp::{
        params::get_file_content_params::GetFileContentParams,
        responses::{McpResponse, get_file_content_response::GetFileContentResponse},
        server::AppState,
    },
    utils,
};

/// Tool for retrieving file content.
#[derive(Clone)]
pub struct GetFileContentTool {
    state: Arc<AppState>,
}

impl GetFileContentTool {
    /// Creates a new instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the file content retrieval.
    pub async fn execute(&self, params: GetFileContentParams) -> Result<CallToolResult, ErrorData> {
        // Normalize path
        let path = utils::mcp_normalize::path_normalize(&self.state.config, &params.path)
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

        // Create tools client for files indices
        let client = self
            .state
            .open_search_client
            .client_tool_files(None)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Query file content
        let (file_path, content) = client
            .get_file_content(&path)
            .await
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

        // Format and return
        let response = GetFileContentResponse {
            path: file_path,
            content,
        };
        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
