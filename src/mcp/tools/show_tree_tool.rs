//! show_tree tool handler.
//! Returns directory tree structure for a source.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::mcp::params::show_tree_params::ShowTreeParams;
use crate::mcp::responses::McpResponse;
use crate::mcp::responses::show_tree_response::ShowTreeResponse;
use crate::mcp::server::AppState;
use crate::utils::mcp_normalize::number_normalize;
use crate::utils::mcp_normalize::target_normalize;

/// Tool for generating directory tree structure.
#[derive(Clone)]
pub struct ShowTreeTool {
    state: Arc<AppState>,
}

impl ShowTreeTool {
    /// Creates a new ShowTreeTool instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the tree generation.
    pub async fn execute(&self, params: ShowTreeParams) -> Result<CallToolResult, ErrorData> {
        // Normalize target
        let target = target_normalize(&self.state.config, params.target.as_deref())
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

        // Normalize level
        let level = number_normalize(params.level, ShowTreeParams::default_level(), None);

        // Create tools client
        let client = self
            .state
            .open_search_client
            .client_tool_files_meta(target.clone())
            .await
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

        // Fetch tree
        let output = client
            .show_tree(level)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Build response
        let response = ShowTreeResponse::try_from(output)
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
