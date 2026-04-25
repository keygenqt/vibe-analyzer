//! Tool for searching documentation across all projects.
//! Handles docs search queries.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::configs::app::IndexType;
use crate::mcp::server::AppState;
use crate::mcp::tools::data::params::mcp_search_documentation_params::McpSearchDocumentationParams;
use crate::mcp::tools::data::responses::mcp_search_documentation_response::McpSearchDocumentationResponse;
use crate::utils::mcp_normalize::number_normalize;
use crate::utils::mcp_normalize::query_normalize;

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

    /// Executes the search.
    pub async fn execute(
        &self,
        params: McpSearchDocumentationParams,
    ) -> Result<CallToolResult, ErrorData> {
        let indexes = self.state.config.get_indexes(IndexType::Analysis);
        let query = query_normalize(&params.query);
        let limit = number_normalize(params.limit, McpSearchDocumentationParams::default_limit());

        let results = self
            .state
            .search_client
            .search_documentation(&indexes, query, limit)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            McpSearchDocumentationResponse {
                query: params.query,
                results: results.into_iter().map(|r| r.into()).collect(),
            }
            .format(),
        )]))
    }
}
