//! sync_index tool handler.
//! Triggers incremental indexing for all knowledge sources.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::cli::scan::index_enriches;
use crate::mcp::server::AppState;
use crate::mcp::tools::data::responses::mcp_sync_index_response::McpSyncIndexResponse;

static INDEXING: AtomicBool = AtomicBool::new(false);

/// Tool for triggering source indexing.
#[derive(Clone)]
pub struct SyncIndexTool {
    state: Arc<AppState>,
}

impl SyncIndexTool {
    /// Creates a new SyncIndexTool instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the indexing process for all sources.
    pub async fn execute(&self) -> Result<CallToolResult, ErrorData> {
        if INDEXING.swap(true, Ordering::SeqCst) {
            return Ok(CallToolResult::success(vec![Content::text(
                McpSyncIndexResponse::already_running().format(),
            )]));
        }

        let config = self.state.config.clone();

        tokio::spawn(async move {
            match index_enriches(&config, None, false).await {
                Ok(report) => {
                    tracing::info!(
                        "Indexing completed. Sources: {}, Files: {}, Analysis: {}",
                        report.meta.indexed,
                        report.files.files_indexed,
                        report.files.analysis_indexed
                    );
                }
                Err(e) => {
                    tracing::error!("Indexing failed: {}", e);
                }
            }
            INDEXING.store(false, Ordering::SeqCst);
        });

        Ok(CallToolResult::success(vec![Content::text(
            McpSyncIndexResponse::started().format(),
        )]))
    }
}
