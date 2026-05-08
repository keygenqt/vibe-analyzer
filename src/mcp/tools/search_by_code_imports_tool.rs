//! Tool for searching import statements.
//! Handles code-level import queries.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::mcp::params::search_by_code_imports_params::SearchByCodeImportsParams;
use crate::mcp::responses::McpResponse;
use crate::mcp::responses::search_by_code_imports_response::SearchByCodeImportsMetaResponse;
use crate::mcp::responses::search_by_code_imports_response::SearchByCodeImportsResponse;
use crate::mcp::server::AppState;
use crate::utils::mcp_normalize::number_normalize;
use crate::utils::mcp_normalize::query_normalize;
use crate::utils::mcp_normalize::target_normalize;

/// Tool for searching import statements.
#[derive(Clone)]
pub struct SearchByCodeImportsTool {
    state: Arc<AppState>,
}

impl SearchByCodeImportsTool {
    /// Creates a new instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the search for import statements.
    pub async fn execute(
        &self,
        params: SearchByCodeImportsParams,
    ) -> Result<CallToolResult, ErrorData> {
        // Normalize parameters
        let target = target_normalize(&self.state.config, params.target.as_deref())
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
        let query = query_normalize(&params.query);
        let limit = number_normalize(
            params.limit,
            SearchByCodeImportsParams::default_limit(),
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
            .search_by_code_imports(query, limit)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Convert results
        let results: Vec<SearchByCodeImportsMetaResponse> = files
            .iter()
            .map(SearchByCodeImportsMetaResponse::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Build response
        let response = SearchByCodeImportsResponse {
            query: params.query,
            results,
        };

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
