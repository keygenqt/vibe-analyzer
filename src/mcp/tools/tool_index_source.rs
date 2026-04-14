// src/mcp/tools/index_source.rs

//! index_source tool handler.
//! Triggers incremental indexing for a knowledge source.

use std::collections::HashSet;
use std::sync::Mutex;

use serde_json::Value;
use serde_json::json;

use crate::cli::scan::index_enriches;
use crate::mcp::api::mcp_response::McpResponse;
use crate::mcp::server::AppState;

lazy_static::lazy_static! {
    static ref INDEXING_LOCKS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

/// Execute index_source tool.
pub async fn call(id: Option<Value>, args: &Value, state: &AppState) -> McpResponse {
    // Extract target source name from arguments
    let target = match args.get("target").and_then(|t| t.as_str()) {
        Some(t) => t.to_string(),
        None => return McpResponse::invalid_params(id),
    };

    // Check if already indexing this source
    {
        let mut locks = INDEXING_LOCKS.lock().unwrap();
        if locks.contains(&target) {
            return McpResponse::success(
                id,
                json!({
                    "status": "already_indexing",
                    "target": target,
                }),
            );
        }
        locks.insert(target.clone());
    }

    // Clone values for background task
    let config = state.config.clone();
    let target_clone = target.clone();
    let target_for_response = target.clone();

    // Run indexing in background
    tokio::spawn(async move {
        match index_enriches(&config, Some(target_clone), false).await {
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
        // Release lock
        INDEXING_LOCKS.lock().unwrap().remove(&target);
    });

    McpResponse::success(
        id,
        json!({
            "status": "indexing_started",
            "target": target_for_response,
        }),
    )
}
