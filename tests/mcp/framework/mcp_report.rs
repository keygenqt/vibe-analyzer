//! MCP test report generator.
//! Builds structured JSON reports from test data.

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;

/// Complete test report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpReport {
    pub test_name: String,
    pub model: String,
    pub timestamp: String,
    pub queries: Vec<QueryReport>,
    pub summary: ReportSummary,
}

/// Single query report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryReport {
    pub query: String,
    pub tool_calls: Vec<ToolCall>,
    pub response: String,
    pub duration_ms: u64,
    pub timestamp: String,
}

/// Tool call details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub args: String,
    pub result: String,
}

/// Report summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReportSummary {
    pub total_queries: usize,
    pub successful_tool_calls: usize,
    pub total_duration_ms: u64,
    pub avg_response_time_ms: u64,
}

impl McpReport {
    /// Creates a new report from test data.
    pub fn new(test_name: String, model: String, queries: Vec<QueryReport>) -> Self {
        let total_queries = queries.len();
        let successful_tool_calls = queries.iter().filter(|q| !q.tool_calls.is_empty()).count();
        let total_duration_ms = queries.iter().map(|q| q.duration_ms).sum();
        let avg_response_time_ms = if total_queries > 0 {
            total_duration_ms / total_queries as u64
        } else {
            0
        };

        Self {
            test_name,
            model,
            timestamp: chrono::Utc::now().to_rfc3339(),
            queries,
            summary: ReportSummary {
                total_queries,
                successful_tool_calls,
                total_duration_ms,
                avg_response_time_ms,
            },
        }
    }

    /// Saves report to JSON file (overwrites if exists).
    pub fn save(&self, output_dir: &Path) -> Result<()> {
        std::fs::create_dir_all(output_dir)?;
        let filename = format!("report_{}_{}.json", self.model, self.test_name);
        let path = output_dir.join(filename);
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, json)?;
        tracing::trace!("Report saved to {}", path.display());
        Ok(())
    }
}
