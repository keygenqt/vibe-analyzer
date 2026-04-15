//! get_stats tool handler.
//! Returns project statistics by language.

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;

use crate::configs::app::IndexType;
use crate::mcp::server::AppState;

/// Parameters for the get_stats tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetStatsParams {
    /// Target project name (all if not specified).
    pub target: Option<String>,
}

/// Tool for retrieving project statistics.
#[derive(Clone)]
pub struct GetStatsTool {
    state: Arc<AppState>,
}

impl GetStatsTool {
    /// Creates a new GetStatsTool instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the statistics query.
    pub async fn get_stats(&self, params: GetStatsParams) -> Result<CallToolResult, ErrorData> {
        let indexes = match &params.target {
            Some(target) => {
                vec![
                    self.state
                        .config
                        .get_index(target, IndexType::Analysis)
                        .map_err(|e| {
                            ErrorData::invalid_params(
                                e.to_string(),
                                Some(json!({ "detail": e.to_string() })),
                            )
                        })?,
                ]
            }
            None => self.state.config.get_indexes(IndexType::Analysis),
        };

        let results = self
            .state
            .search_client
            .get_stats(&indexes)
            .await
            .map_err(|e| {
                ErrorData::internal_error(e.to_string(), Some(json!({ "detail": e.to_string() })))
            })?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(&results).unwrap(),
        )]))
    }
}
