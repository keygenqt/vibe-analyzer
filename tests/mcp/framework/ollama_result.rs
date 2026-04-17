//! MCP test result output.
//! Defines the result structure returned by test execution.

use serde::{Deserialize, Serialize};

/// Result of running a test scenario with a specific model.
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaResult {
    /// Tool name from scenario.
    pub tool: String,
    /// Model used for testing.
    pub model: String,
    /// Response text from the model.
    pub response: String,
}
