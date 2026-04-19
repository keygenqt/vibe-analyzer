//! Request parameters for the get_tree tool.
//! Defines the structure for directory tree exploration requests.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Parameters for the get_tree tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetTreeParams {
    /// Target project path (from list_projects).
    #[serde(default)]
    pub target: Option<String>,
    /// Maximum depth (default: 3, max: 10).
    #[serde(default = "GetTreeParams::default_level")]
    pub level: i32,
}

impl GetTreeParams {
    /// Default tree depth.
    pub const fn default_level() -> i32 {
        3
    }
}
