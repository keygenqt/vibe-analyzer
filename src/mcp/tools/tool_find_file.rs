//! find_file tool handler.
//! Searches files by path pattern in a project.

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

/// Parameters for the find_file tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FindFileParams {
    /// Path pattern to search.
    pub pattern: String,
    /// Target project name.
    pub target: String,
    /// Maximum results (default: 10).
    #[serde(default = "default_limit")]
    pub limit: usize,
}

/// Default limit for search results.
fn default_limit() -> usize {
    10
}

/// Tool for searching files by path pattern.
#[derive(Clone)]
pub struct FindFileTool {
    state: Arc<AppState>,
}

impl FindFileTool {
    /// Creates a new FindFileTool instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the file search.
    pub async fn find_file(&self, params: FindFileParams) -> Result<CallToolResult, ErrorData> {
        let index = self
            .state
            .config
            .get_index(&params.target, IndexType::Files)
            .map_err(|e| {
                ErrorData::invalid_params(e.to_string(), Some(json!({ "detail": e.to_string() })))
            })?;

        let results = self
            .state
            .search_client
            .get_files(&params.pattern, &index, params.limit)
            .await
            .map_err(|e| {
                ErrorData::internal_error(e.to_string(), Some(json!({ "detail": e.to_string() })))
            })?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(&results).unwrap(),
        )]))
    }
}
