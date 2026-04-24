//! show_stats tool handler.
//! Returns project statistics by language.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::configs::app::IndexType;
use crate::mcp::server::AppState;
use crate::mcp::tools::data::params::mcp_show_stats_params::McpShowStatsParams;
use crate::mcp::tools::data::responses::mcp_show_stats_response::McpShowStatsResponse;
use crate::utils::mcp_normalize::get_targets_normalize;

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
    pub async fn execute(&self, params: McpShowStatsParams) -> Result<CallToolResult, ErrorData> {
        // Normalize target and get indexes
        let indexes =
            get_targets_normalize(&self.state.config, &params.target, IndexType::Analysis)
                .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

        // Fetch stats from OpenSearch
        let results = self
            .state
            .search_client
            .show_stats_search(&indexes)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Build and return response
        Ok(CallToolResult::success(vec![Content::text(
            McpShowStatsResponse {
                target: params.target.clone(),
                languages: results.languages.into_iter().map(|l| l.into()).collect(),
                total: results.total.into(),
            }
            .format(),
        )]))
    }
}
