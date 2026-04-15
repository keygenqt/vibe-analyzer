//! list_sources tool handler.
//! Returns all indexed projects with basic metadata.

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;
use serde_json::json;
use std::sync::Arc;

use crate::mcp::server::AppState;

/// Tool for listing all knowledge sources.
#[derive(Clone)]
pub struct ListSourcesTool {
    state: Arc<AppState>,
}

impl ListSourcesTool {
    /// Creates a new ListSourcesTool instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Returns metadata for all indexed projects.
    pub async fn list_sources(&self) -> Result<CallToolResult, ErrorData> {
        let results = self.state.search_client.list_meta().await.map_err(|e| {
            ErrorData::internal_error(e.to_string(), Some(json!({ "detail": e.to_string() })))
        })?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(&results).unwrap(),
        )]))
    }
}
