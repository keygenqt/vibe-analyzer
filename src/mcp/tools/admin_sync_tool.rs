//! admin_sync tool handler.
//! Triggers incremental indexing for all knowledge sources.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::indexer::scan_index;
use crate::mcp::responses::McpResponse;
use crate::mcp::responses::admin_sync_response::AdminSyncResponse;
use crate::mcp::server::AppState;

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
                AdminSyncResponse::already_running().format(),
            )]));
        }
        let config = self.state.config.clone();
        tokio::spawn(async move {
            let _ = scan_index(&config, None, false).await;
            INDEXING.store(false, Ordering::SeqCst);
        });
        Ok(CallToolResult::success(vec![Content::text(
            AdminSyncResponse::started().format(),
        )]))
    }
}
