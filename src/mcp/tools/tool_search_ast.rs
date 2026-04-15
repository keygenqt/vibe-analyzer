//! search_ast tool handler.
//! Searches in code structure (functions, classes, imports).

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

/// Parameters for the search_ast tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchAstParams {
    /// Search query.
    pub query: String,
    /// Target project name (optional).
    pub target: Option<String>,
    /// Maximum results (default: 10).
    #[serde(default = "default_limit")]
    pub limit: usize,
}

/// Default limit for search results.
fn default_limit() -> usize {
    10
}

/// Tool for searching AST elements.
#[derive(Clone)]
pub struct SearchAstTool {
    state: Arc<AppState>,
}

impl SearchAstTool {
    /// Creates a new SearchAstTool instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the AST search.
    pub async fn search_ast(&self, params: SearchAstParams) -> Result<CallToolResult, ErrorData> {
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
            .search_ast(&indexes, &params.query, params.limit)
            .await
            .map_err(|e| {
                ErrorData::internal_error(e.to_string(), Some(json!({ "detail": e.to_string() })))
            })?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(&results).unwrap(),
        )]))
    }
}
