//! Search result structure for project metadata queries.
//! Contains root path, LLM-generated summary, license, and statistics.

use serde::Deserialize;
use serde::Serialize;

use crate::scanner::data::output::scan_license_output::ScanLicenseOutput;
use crate::scanner::data::output::scan_project_stats_output::ScanProjectStatsOutput;

/// Project metadata returned from search queries.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Meta")]
pub struct OpensearchProjectMetaResponse {
    /// Absolute path to source root.
    pub root: String,
    /// LLM-generated project summary.
    pub summary: String,
    /// Detected license information (if found).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<ScanLicenseOutput>,
    /// Aggregated source statistics.
    pub stats: ScanProjectStatsOutput,
}
