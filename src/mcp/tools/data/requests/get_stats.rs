//! Request parameters for the get_stats tool.
//! Defines the structure for project statistics requests.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Parameters for the get_stats tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetStatsParams {
    /// Target project name (from list_projects). If not specified, returns stats for all projects.
    #[serde(default)]
    pub target: Option<String>,
}
