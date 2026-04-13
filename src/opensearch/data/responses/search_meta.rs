//! Search result structure for project metadata queries.
//! Contains root path, LLM-generated summary, license, and statistics.

use serde::Deserialize;
use serde::Serialize;

use crate::scanner::data::output::license_info::LicenseInfo;
use crate::scanner::data::output::source_stats::SourceStats;

/// Project metadata returned from search queries.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchMeta {
    /// Absolute path to source root.
    pub root: String,
    /// LLM-generated project summary.
    pub summary: String,
    /// Detected license information (if found).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseInfo>,
    /// Aggregated source statistics.
    pub stats: SourceStats,
}
