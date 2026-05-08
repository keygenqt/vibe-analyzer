//! show_stats tool handler.
//! Returns project statistics by language.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::mcp::params::show_stats_params::ShowStatsParams;
use crate::mcp::responses::McpResponse;
use crate::mcp::responses::show_stats_response::ShowStatsResponse;
use crate::mcp::server::AppState;
use crate::utils::mcp_normalize::target_normalize;

/// Tool for retrieving project statistics.
#[derive(Clone)]
pub struct ShowStatsTool {
    state: Arc<AppState>,
}

impl ShowStatsTool {
    /// Creates a new ShowStatsTool instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the statistics query.
    pub async fn execute(&self, params: ShowStatsParams) -> Result<CallToolResult, ErrorData> {
        // Normalize target
        let target = target_normalize(&self.state.config, params.target.as_deref())
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

        // Create tools client
        let client = self
            .state
            .open_search_client
            .client_tool_files_meta(target.clone())
            .await
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

        // Fetch stats
        let output = client
            .show_stats_search()
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Build response
        let response = ShowStatsResponse::try_from(output)
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
