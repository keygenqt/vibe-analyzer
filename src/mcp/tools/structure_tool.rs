//! structure tool handler.
//! Returns directory tree structure for a source.

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;
use serde_json::json;
use std::sync::Arc;

use crate::configs::app::IndexType;
use crate::mcp::server::AppState;
use crate::mcp::tools::data::params::mcp_structure_params::McpStructureParams;
use crate::mcp::tools::data::responses::mcp_structure_response::McpStructureResponse;
use crate::utils::mcp_normalize::number_normalize;
use crate::utils::mcp_normalize::target_normalize;

/// Tool for generating directory tree structure.
#[derive(Clone)]
pub struct StructureTool {
    state: Arc<AppState>,
}

impl StructureTool {
    /// Creates a new StructureTool instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the tree generation.
    pub async fn execute(&self, params: McpStructureParams) -> Result<CallToolResult, ErrorData> {
        // Normalize target
        let target = target_normalize(&self.state.config, params.target.as_deref())?;

        // Clamp level to safe range: <=0 → 3 (default), >10 → 10 (max)
        let level = number_normalize(params.level, 1, 10, McpStructureParams::default_level());

        // Get index
        let index = self
            .state
            .config
            .get_index(&target, IndexType::Files)
            .map_err(|e| {
                ErrorData::invalid_params(e.to_string(), Some(json!({ "detail": e.to_string() })))
            })?;

        // Get tree
        let result = self
            .state
            .search_client
            .structure(&index, level as usize)
            .await
            .map_err(|e| {
                ErrorData::internal_error(e.to_string(), Some(json!({ "detail": e.to_string() })))
            })?;

        // Response
        Ok(CallToolResult::success(vec![Content::text(
            McpStructureResponse {
                target: target,
                tree: result.tree,
                total_files: result.files,
                total_dirs: result.directories,
            }
            .format(),
        )]))
    }
}
