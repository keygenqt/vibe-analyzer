//! Tool for searching variables and enums.
//! Handles code-level variable and enum search queries.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::mcp::params::search_by_code_variables_params::SearchByCodeVariablesParams;
use crate::mcp::responses::McpResponse;
use crate::mcp::responses::search_by_code_variables_response::SearchByCodeVariablesMetaResponse;
use crate::mcp::responses::search_by_code_variables_response::SearchByCodeVariablesResponse;
use crate::mcp::server::AppState;
use crate::utils::normalize::number_normalize;
use crate::utils::normalize::query_normalize;
use crate::utils::normalize::target_normalize;

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

    /// Executes the search for variables and enums.
    pub async fn execute(
        &self,
        params: SearchByCodeVariablesParams,
    ) -> Result<CallToolResult, ErrorData> {
        // Normalize parameters
        let target = target_normalize(&self.state.config, params.target.as_deref())
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
        let query = query_normalize(&params.query);
        let limit = number_normalize(
            params.limit,
            SearchByCodeVariablesParams::default_limit(),
            None,
        );

        // Create tools client
        let client = self
            .state
            .open_search_client
            .client_tool_files_meta(target)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Query OpenSearch
        let files = client
            .search_by_code_variables(query, limit)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Convert results
        let results: Vec<SearchByCodeVariablesMetaResponse> = files
            .iter()
            .map(SearchByCodeVariablesMetaResponse::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Build response
        let response = SearchByCodeVariablesResponse {
            query: params.query,
            results,
        };

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
