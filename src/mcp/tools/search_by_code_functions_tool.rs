//! Tool for searching functions and methods.
//! Handles code-level function search queries.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::configs::app::IndexType;
use crate::mcp::server::AppState;
use crate::mcp::tools::data::params::mcp_search_by_code_functions_params::McpSearchByCodeFunctionsParams;
use crate::mcp::tools::data::responses::mcp_search_by_code_functions_response::McpSearchByCodeFunctionsResponse;
use crate::utils::mcp_normalize::get_targets_normalize;
use crate::utils::mcp_normalize::number_normalize;
use crate::utils::mcp_normalize::query_normalize;

/// Tool for searching functions and methods.
#[derive(Clone)]
pub struct SearchByCodeFunctionsTool {
    state: Arc<AppState>,
}

impl SearchByCodeFunctionsTool {
    /// Creates a new instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the search.
    pub async fn execute(
        &self,
        params: McpSearchByCodeFunctionsParams,
    ) -> Result<CallToolResult, ErrorData> {
        let config = &self.state.config;
        let indexes = get_targets_normalize(config, &params.target, IndexType::Analysis)
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
        let query = query_normalize(&params.query);
        let limit = number_normalize(
            params.limit,
            McpSearchByCodeFunctionsParams::default_limit(),
        );

        let results = self
            .state
            .search_client
            .search_by_code_functions(&indexes, query, limit)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            McpSearchByCodeFunctionsResponse {
                query: params.query,
                results: results.into_iter().map(|r| r.into()).collect(),
            }
            .format(),
        )]))
    }
}
