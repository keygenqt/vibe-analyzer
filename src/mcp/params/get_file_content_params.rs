//! Request parameters for the get_file_content tool.
//! Defines the structure for file content retrieval requests.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Parameters for the get_file_content tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetFileContentParams {
    /// File path (required, relative or absolute).
    pub path: String,
}
