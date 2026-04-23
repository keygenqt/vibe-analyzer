//! stats tool handler.
//! Returns project statistics by language.

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;
use serde_json::json;
use std::sync::Arc;

use crate::configs::app::IndexType;
use crate::mcp::server::AppState;
use crate::mcp::tools::data::params::mcp_stats_params::McpStatsParams;
use crate::mcp::tools::data::responses::mcp_stats_response::McpStatLanguageResponse;
use crate::mcp::tools::data::responses::mcp_stats_response::McpStatTotalResponse;
use crate::mcp::tools::data::responses::mcp_stats_response::McpStatsResponse;
use crate::utils::mcp_normalize::get_targets_normalize;

/// Tool for retrieving project statistics.
#[derive(Clone)]
pub struct StatsTool {
    state: Arc<AppState>,
}

impl StatsTool {
    /// Creates a new StatsTool instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the statistics query.
    pub async fn execute(&self, params: McpStatsParams) -> Result<CallToolResult, ErrorData> {
        // Normalize target and get list indexes
        let indexes =
            get_targets_normalize(&self.state.config, &params.target, IndexType::Analysis)?;

        // Get stats from OpenSearch
        let results = self
            .state
            .search_client
            .stats(&indexes)
            .await
            .map_err(|e| {
                ErrorData::internal_error(e.to_string(), Some(json!({ "detail": e.to_string() })))
            })?;

        // Build response
        let response = McpStatsResponse {
            target: params.target.clone(),
            languages: results
                .languages
                .into_iter()
                .map(|l| McpStatLanguageResponse {
                    language: l.language,
                    files: l.files,
                    lines: l.lines,
                    ast_objects: l.ast_objects,
                    size_human: l.size_human,
                })
                .collect(),
            total: McpStatTotalResponse {
                files: results.total.files,
                lines: results.total.lines,
                ast_objects: results.total.ast_objects,
                size_human: results.total.size_human,
            },
        };

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
