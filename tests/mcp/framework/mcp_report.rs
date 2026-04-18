//! MCP test report generator.
//! Builds structured JSON reports from test data.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Complete test report.
#[derive(Debug, Serialize, Deserialize)]
pub struct McpReport {
    pub test_name: String,
    pub model: String,
    pub timestamp: String,
    pub queries: Vec<QueryReport>,
    pub summary: ReportSummary,
}

/// Single query report.
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryReport {
    pub query: String,
    pub tool_calls: Vec<ToolCall>,
    pub response: String,
    pub duration_ms: u64,
    pub timestamp: String,
}

/// Tool call details.
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub args: String,
}

/// Report summary.
#[derive(Debug, Serialize, Deserialize, Default)]
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

        let filename = format!("report_{}_{}.json", self.test_name, self.model);
        let path = output_dir.join(filename);

        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, json)?;

        tracing::info!("Report saved to {}", path.display());
        Ok(())
    }

    /// Prints a formatted summary.
    pub fn print_summary(&self) {
        let separator1 = "━".repeat(79);
        let separator2 = "─".repeat(79);
        println!();
        println!("{}", separator1);
        println!("MCP Test Report: {}", self.test_name);
        println!("{}", separator1);
        println!("Model: {}", self.model);
        println!("Timestamp: {}", self.timestamp);
        for (i, q) in self.queries.iter().enumerate() {
            println!("{}", separator2);
            println!("[{}] {}", i + 1, q.query);
            println!("{}", separator2);
            println!("{}", q.response);
        }
        println!("{}", separator1);
        println!("Summary:");
        println!("  Total queries: {}", self.summary.total_queries);
        println!(
            "  Successful tool calls: {}",
            self.summary.successful_tool_calls
        );
        println!("  Total duration: {} ms", self.summary.total_duration_ms);
        println!(
            "  Avg response time: {} ms",
            self.summary.avg_response_time_ms
        );
        println!("{}", separator1);
        println!();
    }
}
