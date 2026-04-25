//! Request parameters for the get_file_ast tool.
//! Defines the structure for AST retrieval requests.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Parameters for the get_file_ast tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "FileAst")]
pub struct McpGetFileAstParams {
    /// File path (required, relative or absolute).
    pub path: String,
}
