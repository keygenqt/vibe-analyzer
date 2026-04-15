//! find_source tool handler.
//! Searches knowledge sources by name, license, or description.

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;

use crate::mcp::server::AppState;

/// Parameters for the find_source tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FindSourceParams {
    #[schemars(description = "Search term: project name, license type, or topic keywords")]
    pub query: String,

    #[schemars(
        description = "Maximum results. Default is 1. Increase if multiple matches expected"
    )]
    #[serde(default = "default_limit")]
    pub limit: usize,
}

/// Default limit for search results.
fn default_limit() -> usize {
    1
}

/// Tool for searching knowledge sources.
#[derive(Clone)]
pub struct FindSourceTool {
    state: Arc<AppState>,
}

impl FindSourceTool {
    /// Creates a new FindSourceTool instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the source search.
    pub async fn find_source(&self, params: FindSourceParams) -> Result<CallToolResult, ErrorData> {
        let results = self
            .state
            .search_client
            .search_meta(&params.query, params.limit)
            .await
            .map_err(|e| {
                ErrorData::internal_error(e.to_string(), Some(json!({ "detail": e.to_string() })))
            })?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(&results).unwrap(),
        )]))
    }
}
