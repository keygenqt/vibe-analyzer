//! Response structure for the admin_sync tool.

use serde::Serialize;

use crate::mcp::responses::McpResponse;

/// Response from admin_sync tool.
#[derive(Debug, Serialize)]
pub struct AdminSyncResponse {
    /// Request result.
    pub result: AdminSyncResult,
    /// User message.
    pub message: String,
}

/// Index request outcome.
#[derive(Debug, Serialize)]
pub enum AdminSyncResult {
    /// Started.
    Started,
    /// Already running.
    AlreadyRunning,
}

impl AdminSyncResponse {
    /// Indexing started successfully.
    pub fn started() -> Self {
        Self {
            result: AdminSyncResult::Started,
            message: "Indexing started. Projects are updating now.".to_string(),
        }
    }

    /// Indexing is already in progress.
    pub fn already_running() -> Self {
        Self {
            result: AdminSyncResult::AlreadyRunning,
            message: "Indexing is already running. Please wait.".to_string(),
        }
    }
}

impl McpResponse for AdminSyncResponse {
    /// Returns a formatted tool result with JSON data.
    fn format(&self) -> String {
        format!(
            "[indexing {}]\nUse this data to respond to the user.\n\n{}",
            match self.result {
                AdminSyncResult::Started => "started",
                AdminSyncResult::AlreadyRunning => "already running",
            },
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
