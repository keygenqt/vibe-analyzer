//! Tool for searching classes, structs, and interfaces.
//! Handles code-level type search queries.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::mcp::params::search_by_code_classes_params::SearchByCodeClassesParams;
use crate::mcp::responses::McpResponse;
use crate::mcp::responses::search_by_code_classes_response::SearchByCodeClassesMetaResponse;
use crate::mcp::responses::search_by_code_classes_response::SearchByCodeClassesResponse;
use crate::mcp::server::AppState;
use crate::utils::normalize::number_normalize;
use crate::utils::normalize::query_normalize;
use crate::utils::normalize::target_normalize;

/// Tool for searching classes, structs, and interfaces.
#[derive(Clone)]
pub struct SearchByCodeClassesTool {
    state: Arc<AppState>,
}

impl SearchByCodeClassesTool {
    /// Creates a new instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the search for classes, structs, and interfaces.
    pub async fn execute(
        &self,
        params: SearchByCodeClassesParams,
    ) -> Result<CallToolResult, ErrorData> {
        // Normalize parameters
        let target = target_normalize(&self.state.config, params.target.as_deref())
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
        let query = query_normalize(&params.query);
        let limit = number_normalize(
            params.limit,
            SearchByCodeClassesParams::default_limit(),
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
            .search_by_code_classes(query, limit)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Convert results
        let results: Vec<SearchByCodeClassesMetaResponse> = files
            .iter()
            .map(SearchByCodeClassesMetaResponse::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Build response
        let response = SearchByCodeClassesResponse {
            query: params.query,
            results,
        };

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
