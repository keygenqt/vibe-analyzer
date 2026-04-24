//! Request parameters for the stats tool.
//! Defines the structure for project statistics requests.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Parameters for the stats tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "Stats")]
pub struct McpShowStatsParams {
    /// Target project (optional) - project name.
    #[serde(default)]
    pub target: Option<String>,
}
