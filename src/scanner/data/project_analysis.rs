//! Project-level analysis results
//!
//! This module defines [`ProjectAnalysis`] containing complete
//! analysis data for a single project.

use super::file_analysis::FileAnalysis;
use super::license_info::LicenseInfo;
use super::project_stats::ProjectStats;
use super::readme_info::ReadmeInfo;
use serde::{Deserialize, Serialize};

/// Complete analysis results for a single project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAnalysis {
    /// Absolute path to project root
    pub project: String,
    /// ISO 8601 timestamp of analysis
    pub indexed_at: String,
    /// README file content (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<ReadmeInfo>,
    /// Detected license information (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseInfo>,
    /// Aggregated project statistics
    pub stats: ProjectStats,
    /// Per-file analysis results
    pub files: Vec<FileAnalysis>,
}
