//! Source-level analysis results.
//! Contains complete scan data for a single source including metadata, statistics, and file details.

use super::scan_file_analysis_output::ScanFileAnalysisOutput;
use super::scan_license_output::ScanLicenseOutput;
use super::scan_project_stats_output::ScanProjectStatsOutput;
use super::scan_readme_output::ScanReadmeOutput;
use serde::Deserialize;
use serde::Serialize;

/// Complete analysis results for a single source
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "ProjectAnalysis")]
pub struct ScanProjectAnalysisOutput {
    /// Absolute path to source root
    pub root: String,
    /// ISO 8601 timestamp of analysis
    pub indexed_at: String,
    /// README file content (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<ScanReadmeOutput>,
    /// Detected license information (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<ScanLicenseOutput>,
    /// Aggregated source statistics
    pub stats: ScanProjectStatsOutput,
    /// Per-file analysis results
    pub files: Vec<ScanFileAnalysisOutput>,
}
