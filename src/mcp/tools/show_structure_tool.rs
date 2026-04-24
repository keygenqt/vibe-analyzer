//! show_structure tool handler.
//! Returns directory tree structure for a source.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::configs::app::IndexType;
use crate::mcp::server::AppState;
use crate::mcp::tools::data::params::mcp_show_structure_params::McpShowStructureParams;
use crate::mcp::tools::data::responses::mcp_show_structure_response::McpShowStructureResponse;
use crate::utils::mcp_normalize::get_targets_normalize;
use crate::utils::mcp_normalize::number_normalize;

/// Tool for generating directory tree structure.
#[derive(Clone)]
pub struct ShowStructureTool {
    state: Arc<AppState>,
}

impl ShowStructureTool {
    /// Creates a new ShowStructureTool instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the tree generation.
    pub async fn execute(
        &self,
        params: McpShowStructureParams,
    ) -> Result<CallToolResult, ErrorData> {
        // Normalize target and level
        let indexes =
            get_targets_normalize(&self.state.config, &params.target, IndexType::Analysis)
                .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
        let level = number_normalize(params.level, McpShowStructureParams::default_level());

        // Fetch structure from OpenSearch
        let result = self
            .state
            .search_client
            .show_structure(&indexes, level as usize)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Build and return response
        Ok(CallToolResult::success(vec![Content::text(
            McpShowStructureResponse {
                target: params.target.clone(),
                tree: result.tree,
                total_files: result.files,
                total_dirs: result.directories,
            }
            .format(),
        )]))
    }
}
