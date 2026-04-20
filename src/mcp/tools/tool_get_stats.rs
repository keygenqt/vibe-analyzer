//! get_stats tool handler.
//! Returns project statistics by language.

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;
use serde_json::json;
use std::sync::Arc;

use crate::configs::app::IndexType;
use crate::mcp::server::AppState;
use crate::mcp::tools::data::requests::get_stats::GetStatsParams;
use crate::mcp::tools::data::responses::get_stats::GetStatsResponse;
use crate::mcp::tools::data::responses::get_stats::LanguageStat;
use crate::mcp::tools::data::responses::get_stats::TotalStat;
use crate::utils::mcp_normalize::target_normalize;

/// Tool for retrieving project statistics.
#[derive(Clone)]
pub struct ToolGetStats {
    state: Arc<AppState>,
}

impl ToolGetStats {
    /// Creates a new ToolGetStats instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the statistics query.
    pub async fn execute(&self, params: GetStatsParams) -> Result<CallToolResult, ErrorData> {
        // Normalize target if provided
        let target = match &params.target {
            Some(t) => Some(target_normalize(&self.state.config, Some(t.as_str()))?),
            None => None,
        };

        // Get indexes
        let indexes = match &target {
            Some(t) => vec![
                self.state
                    .config
                    .get_index(t, IndexType::Analysis)
                    .map_err(|e| {
                        ErrorData::invalid_params(
                            e.to_string(),
                            Some(json!({ "detail": e.to_string() })),
                        )
                    })?,
            ],
            None => self.state.config.get_indexes(IndexType::Analysis),
        };

        // Get stats from OpenSearch
        let results = self
            .state
            .search_client
            .get_stats(&indexes)
            .await
            .map_err(|e| {
                ErrorData::internal_error(e.to_string(), Some(json!({ "detail": e.to_string() })))
            })?;

        // Build response
        let response = GetStatsResponse {
            target: params.target.clone(),
            languages: results
                .languages
                .into_iter()
                .map(|l| LanguageStat {
                    language: l.language,
                    files: l.files,
                    lines: l.lines,
                    ast_objects: l.ast_objects,
                    size_human: l.size_human,
                })
                .collect(),
            total: TotalStat {
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
