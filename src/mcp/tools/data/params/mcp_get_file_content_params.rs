//! Request parameters for the get_file_content tool.
//! Defines the structure for file content retrieval requests.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for the get_file_content tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "FileContent")]
pub struct McpGetFileContentParams {
    /// File path.
    pub path: String,
}
