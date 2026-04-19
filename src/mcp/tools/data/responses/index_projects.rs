//! Response structure for the index_projects tool.

use serde::Deserialize;
use serde::Serialize;

/// Response from index_projects tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IndexProjectsResponse {
    /// What happened with the indexing request.
    pub result: IndexResult,
    /// Simple message for the user.
    pub message: String,
}

/// What happened when index was requested.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndexResult {
    /// Indexing just started - it's working now.
    Started,
    /// Indexing was already running - no need to start again.
    AlreadyRunning,
}

impl IndexProjectsResponse {
    /// Indexing started successfully.
    pub fn started() -> Self {
        Self {
            result: IndexResult::Started,
            message: "Indexing started. Projects are updating now.".to_string(),
        }
    }

    /// Indexing is already in progress.
    pub fn already_running() -> Self {
        Self {
            result: IndexResult::AlreadyRunning,
            message: "Indexing is already running. Please wait.".to_string(),
        }
    }

    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        let json = serde_json::to_string_pretty(self).unwrap_or_default();
        let status = match self.result {
            IndexResult::Started => "started",
            IndexResult::AlreadyRunning => "already_running",
        };
        format!(
            "[TOOL RESULT - Indexing status: {}]\n{}\n\nUse this data to respond to the user.",
            status, json
        )
    }
}
