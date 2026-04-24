//! Response structure for the admin_sync tool.

use serde::{Deserialize, Serialize};

/// Response from admin_sync tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "AdminSync")]
pub struct McpAdminSyncResponse {
    /// Request result.
    pub result: McpAdminSyncResult,
    /// User message.
    pub message: String,
}

/// Index request outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "AdminSyncResult")]
pub enum McpAdminSyncResult {
    /// Started.
    Started,
    /// Already running.
    AlreadyRunning,
}

impl McpAdminSyncResponse {
    /// Indexing started successfully.
    pub fn started() -> Self {
        Self {
            result: McpAdminSyncResult::Started,
            message: "Indexing started. Projects are updating now.".to_string(),
        }
    }

    /// Indexing is already in progress.
    pub fn already_running() -> Self {
        Self {
            result: McpAdminSyncResult::AlreadyRunning,
            message: "Indexing is already running. Please wait.".to_string(),
        }
    }

    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        format!(
            "[indexing {}]\n\nUse this data to respond to the user.\n{}",
            match self.result {
                McpAdminSyncResult::Started => "started",
                McpAdminSyncResult::AlreadyRunning => "already running",
            },
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
