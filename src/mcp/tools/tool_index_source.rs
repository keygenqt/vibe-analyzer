//! index_source tool handler.
//! Triggers incremental indexing for a knowledge source.

use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

use crate::cli::scan::index_enriches;
use crate::mcp::server::AppState;

lazy_static::lazy_static! {
    static ref INDEXING_LOCKS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

/// Parameters for the index_source tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IndexSourceParams {
    /// Project name.
    pub target: String,
}

/// Tool for triggering source indexing.
#[derive(Clone)]
pub struct IndexSourceTool {
    state: Arc<AppState>,
}

impl IndexSourceTool {
    /// Creates a new IndexSourceTool instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the indexing process.
    pub async fn index_source(
        &self,
        params: IndexSourceParams,
    ) -> Result<CallToolResult, ErrorData> {
        let target = params.target;

        {
            let mut locks = INDEXING_LOCKS.lock().unwrap();
            if locks.contains(&target) {
                return Ok(CallToolResult::success(vec![Content::text(
                    json!({ "status": "already_indexing", "target": target }).to_string(),
                )]));
            }
            locks.insert(target.clone());
        }

        let config = self.state.config.clone();
        let target_clone = target.clone();
        let target_for_response = target.clone();

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
            INDEXING_LOCKS.lock().unwrap().remove(&target);
        });

        Ok(CallToolResult::success(vec![Content::text(
            json!({
                "status": "indexing_started",
                "target": target_for_response,
                "message": "Indexing started in background. The knowledge base will be updated shortly."
            }).to_string(),
        )]))
    }
}
