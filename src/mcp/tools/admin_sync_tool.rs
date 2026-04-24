//! admin_sync tool handler.
//! Triggers incremental indexing for all knowledge sources.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::cli::scan::index_enriches;
use crate::mcp::server::AppState;
use crate::mcp::tools::data::responses::mcp_admin_sync_response::McpAdminSyncResponse;

static INDEXING: AtomicBool = AtomicBool::new(false);

/// Tool for triggering source indexing.
#[derive(Clone)]
pub struct AdminSyncTool {
    state: Arc<AppState>,
}

impl AdminSyncTool {
    /// Creates a new AdminSyncTool instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the indexing process for all sources.
    pub async fn execute(&self) -> Result<CallToolResult, ErrorData> {
        if INDEXING.swap(true, Ordering::SeqCst) {
            return Ok(CallToolResult::success(vec![Content::text(
                McpAdminSyncResponse::already_running().format(),
            )]));
        }
        let config = self.state.config.clone();
        tokio::spawn(async move {
            let _ = index_enriches(&config, None, false).await;
            INDEXING.store(false, Ordering::SeqCst);
        });
        Ok(CallToolResult::success(vec![Content::text(
            McpAdminSyncResponse::started().format(),
        )]))
    }
}
