//! Source-level analysis results.
//! Contains complete scan data for a single source including metadata, statistics, and file details.

use super::file_analysis::FileAnalysis;
use super::license_info::LicenseInfo;
use super::readme_info::ReadmeInfo;
use super::source_stats::SourceStats;
use serde::Deserialize;
use serde::Serialize;

/// Complete analysis results for a single source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceAnalysis {
    /// Absolute path to source root
    pub root: String,
    /// ISO 8601 timestamp of analysis
    pub indexed_at: String,
    /// README file content (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<ReadmeInfo>,
    /// Detected license information (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseInfo>,
    /// Aggregated source statistics
    pub stats: SourceStats,
    /// Per-file analysis results
    pub files: Vec<FileAnalysis>,
}
