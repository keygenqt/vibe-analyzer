//! show_tree tool handler.
//! Returns directory tree structure for a source.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::configs::app::IndexType;
use crate::mcp::server::AppState;
use crate::mcp::tools::data::params::mcp_show_tree_params::McpShowTreeParams;
use crate::mcp::tools::data::responses::mcp_show_tree_response::McpShowTreeResponse;
use crate::utils::mcp_normalize::get_targets_normalize;
use crate::utils::mcp_normalize::number_normalize;

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
    pub async fn execute(&self, params: McpShowTreeParams) -> Result<CallToolResult, ErrorData> {
        // Normalize target and level
        let indexes =
            get_targets_normalize(&self.state.config, &params.target, IndexType::Analysis)
                .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
        let level = number_normalize(params.level, McpShowTreeParams::default_level());

        // Fetch structure from OpenSearch
        let result = self
            .state
            .search_client
            .show_tree(&indexes, level as usize)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Build and return response
        Ok(CallToolResult::success(vec![Content::text(
            McpShowTreeResponse {
                target: params.target.clone(),
                tree: result.tree,
                total_files: result.files,
                total_dirs: result.directories,
            }
            .format(),
        )]))
    }
}
