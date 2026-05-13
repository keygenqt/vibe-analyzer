//! Tool for searching documentation across all projects.
//! Handles docs search queries.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::mcp::params::search_documentation_params::SearchDocumentationParams;
use crate::mcp::responses::McpResponse;
use crate::mcp::responses::search_documentation_response::SearchDocumentationMetaResponse;
use crate::mcp::responses::search_documentation_response::SearchDocumentationResponse;
use crate::mcp::server::AppState;
use crate::utils::normalize::number_normalize;
use crate::utils::normalize::query_normalize;

/// Tool for searching documentation.
#[derive(Clone)]
pub struct SearchDocumentationTool {
    state: Arc<AppState>,
}

impl SearchDocumentationTool {
    /// Creates a new instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the search for documentation.
    pub async fn execute(
        &self,
        params: SearchDocumentationParams,
    ) -> Result<CallToolResult, ErrorData> {
        // Normalize parameters
        let query = query_normalize(&params.query);
        let limit = number_normalize(
            params.limit,
            SearchDocumentationParams::default_limit(),
            None,
        );

        // Create tools client (search across all projects)
        let client = self
            .state
            .open_search_client
            .client_tool_files_meta(None)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Query OpenSearch
        let files = client
            .search_documentation(query.clone(), limit)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Convert results
        let results: Vec<SearchDocumentationMetaResponse> = files
            .iter()
            .map(SearchDocumentationMetaResponse::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Build response
        let response = SearchDocumentationResponse {
            query: query.unwrap_or(params.query),
            results,
        };

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
