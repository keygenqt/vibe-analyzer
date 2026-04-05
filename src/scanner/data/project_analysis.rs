use super::file_analysis::FileAnalysis;
use super::license_info::LicenseInfo;
use super::project_stats::ProjectStats;
use super::readme_info::ReadmeInfo;
use serde::{Deserialize, Serialize};

/// Complete analysis results for a single project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAnalysis {
    /// Absolute path to the project root directory
    pub project: String,
    /// ISO 8601 timestamp of when the analysis was performed
    pub indexed_at: String,
    /// README file content (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<ReadmeInfo>,
    /// Detected license information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseInfo>,
    /// Aggregated statistics across all files in the project
    pub stats: ProjectStats,
    /// Detailed analysis for each file in the project
    pub files: Vec<FileAnalysis>,
}
