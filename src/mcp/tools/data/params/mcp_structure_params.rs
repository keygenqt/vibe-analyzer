//! Request parameters for the structure tool.
//! Defines the structure for directory tree exploration requests.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Parameters for the structure tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "Structure")]
pub struct McpStructureParams {
    /// Target project (optional) - project name.
    #[serde(default)]
    pub target: Option<String>,
    /// Maximum depth (default: 3, max: 10).
    #[serde(default = "McpStructureParams::default_level")]
    pub level: i32,
}

impl McpStructureParams {
    /// Default tree depth.
    pub const fn default_level() -> i32 {
        3
    }
}
