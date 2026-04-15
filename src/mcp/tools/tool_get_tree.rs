//! get_tree tool handler.
//! Returns directory tree structure for a source.

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;

use crate::configs::app::IndexType;
use crate::mcp::server::AppState;

/// Parameters for the get_tree tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetTreeParams {
    /// Target project name.
    pub target: String,
    /// Maximum depth (default: 3).
    #[serde(default = "default_level")]
    pub level: usize,
}

/// Default tree depth.
fn default_level() -> usize {
    3
}

/// Tool for generating directory tree structure.
#[derive(Clone)]
pub struct GetTreeTool {
    state: Arc<AppState>,
}

impl GetTreeTool {
    /// Creates a new GetTreeTool instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the tree generation.
    pub async fn get_tree(&self, params: GetTreeParams) -> Result<CallToolResult, ErrorData> {
        let index = self
            .state
            .config
            .get_index(&params.target, IndexType::Files)
            .map_err(|e| {
                ErrorData::invalid_params(e.to_string(), Some(json!({ "detail": e.to_string() })))
            })?;

        let result = self
            .state
            .search_client
            .get_tree(&index, params.level)
            .await
            .map_err(|e| {
                ErrorData::internal_error(e.to_string(), Some(json!({ "detail": e.to_string() })))
            })?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(&result).unwrap(),
        )]))
    }
}
