//! find_code tool handler.
//! Searches code structure (functions, classes, imports) with optional full content.

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;
use serde_json::json;
use std::sync::Arc;
use std::time::Instant;

use crate::analyzer::ollama_cluster;
use crate::configs::app::IndexType;
use crate::mcp::server::AppState;
use crate::mcp::tools::data::requests::find::FindOptions;
use crate::mcp::tools::data::responses::find::FindResponse;

#[derive(Clone)]
pub struct ToolFind {
    state: Arc<AppState>,
}

impl ToolFind {
    /// Creates a new ToolFind instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the find query.
    pub async fn execute(&self, params: FindOptions) -> Result<CallToolResult, ErrorData> {
        let start_total = Instant::now();
        tracing::info!(
            query = %params.query,
            is_keywords = params.is_keywords,
            "🔵 [1/4] Starting find"
        );

        // Get all analysis indexes
        let indexes = self.state.config.get_indexes(IndexType::Analysis);

        // Generate keywords via Ollama
        let keywords = if params.is_keywords {
            tracing::info!("🔵 [2/4] Generating keywords via Ollama");
            let start_keywords = Instant::now();
            let cluster = ollama_cluster::OllamaCluster::new(&self.state.config.ollama);
            match cluster.get_active().await {
                Ok(client) => {
                    let keywords = client.get_search_keywords(&params.query).await;
                    tracing::info!(
                        keywords_count = keywords.len(),
                        keywords = ?keywords,
                        elapsed_ms = start_keywords.elapsed().as_millis(),
                        "🔵 [2/4] Keywords generated"
                    );
                    keywords
                }
                Err(e) => {
                    tracing::warn!(
                        error = %e,
                        elapsed_ms = start_keywords.elapsed().as_millis(),
                        "🟡 [2/4] Failed to get Ollama client, proceeding without keywords"
                    );
                    vec![]
                }
            }
        } else {
            tracing::info!("🔵 [2/4] Keywords generation skipped (is_keywords=false)");
            vec![]
        };

        // Search OpenSearch
        tracing::info!(
            query = %params.query,
            keywords_count = keywords.len(),
            include_content = params.include_content,
            limit = params.limit,
            "🔵 [3/4] Searching OpenSearch"
        );
        let start_search = Instant::now();

        let results = self
            .state
            .search_client
            .search_ast(
                &indexes,
                &params.query,
                &keywords,
                params.limit,
                params.include_content,
            )
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    elapsed_ms = start_search.elapsed().as_millis(),
                    "🔴 [3/4] OpenSearch search failed"
                );
                ErrorData::internal_error(e.to_string(), Some(json!({ "detail": e.to_string() })))
            })?;

        tracing::info!(
            results_count = results.len(),
            elapsed_ms = start_search.elapsed().as_millis(),
            "🔵 [3/4] OpenSearch search completed"
        );

        // Format response
        tracing::info!("🔵 [4/4] Formatting response");
        let response = FindResponse {
            query: params.query,
            results,
            include_content_used: params.include_content,
        };

        tracing::info!(
            total_elapsed_ms = start_total.elapsed().as_millis(),
            "✅ Find completed successfully"
        );

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
