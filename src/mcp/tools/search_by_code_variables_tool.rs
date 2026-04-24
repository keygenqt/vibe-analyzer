//! Tool for searching variables and enums.
//! Handles code-level variable and enum search queries.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::configs::app::IndexType;
use crate::mcp::server::AppState;
use crate::mcp::tools::data::params::mcp_search_by_code_variables_params::McpSearchByCodeVariablesParams;
use crate::mcp::tools::data::responses::mcp_search_by_code_variables_response::McpSearchByCodeVariablesResponse;
use crate::utils::mcp_normalize::get_targets_normalize;
use crate::utils::mcp_normalize::number_normalize;
use crate::utils::mcp_normalize::query_normalize;

/// Tool for searching variables and enums.
#[derive(Clone)]
pub struct SearchByCodeVariablesTool {
    state: Arc<AppState>,
}

impl SearchByCodeVariablesTool {
    /// Creates a new instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the search.
    pub async fn execute(
        &self,
        params: McpSearchByCodeVariablesParams,
    ) -> Result<CallToolResult, ErrorData> {
        let config = &self.state.config;
        let indexes = get_targets_normalize(&config, &params.target, IndexType::Analysis)
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
        let query = query_normalize(&params.query);
        let limit = number_normalize(
            params.limit,
            McpSearchByCodeVariablesParams::default_limit(),
        );

        let results = self
            .state
            .search_client
            .search_by_code_variables(&indexes, query, limit)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            McpSearchByCodeVariablesResponse {
                query: params.query,
                results: results.into_iter().map(|r| r.into()).collect(),
            }
            .format(),
        )]))
    }
}
