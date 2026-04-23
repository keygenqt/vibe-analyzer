//! Response structure for the index_projects tool.

use serde::Deserialize;
use serde::Serialize;

/// Response from sync_index tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "SyncIndex")]
pub struct McpSyncIndexResponse {
    /// Request result.
    pub result: McpSyncIndexResult,
    /// User message.
    pub message: String,
}

/// Index request outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "SyncIndexResult")]
pub enum McpSyncIndexResult {
    /// Started.
    Started,
    /// Already running.
    AlreadyRunning,
}

impl McpSyncIndexResponse {
    /// Indexing started successfully.
    pub fn started() -> Self {
        Self {
            result: McpSyncIndexResult::Started,
            message: "Indexing started. Projects are updating now.".to_string(),
        }
    }

    /// Indexing is already in progress.
    pub fn already_running() -> Self {
        Self {
            result: McpSyncIndexResult::AlreadyRunning,
            message: "Indexing is already running. Please wait.".to_string(),
        }
    }

    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        format!(
            "[indexing {}]\n\nUse this data to respond to the user.\n{}",
            match self.result {
                McpSyncIndexResult::Started => "started",
                McpSyncIndexResult::AlreadyRunning => "already running",
            },
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
