//! Request parameters for the structure tool.
//! Defines the structure for directory tree exploration requests.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Parameters for the structure tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "Structure")]
pub struct McpShowStructureParams {
    /// Target project (optional) - project name.
    #[serde(default)]
    pub target: Option<String>,
    /// Maximum depth (default: 3, max: 10).
    #[serde(default = "McpShowStructureParams::default_level")]
    pub level: Option<i32>,
}

impl McpShowStructureParams {
    /// Default tree depth.
    pub const fn default_level() -> Option<i32> {
        Some(3)
    }
}
