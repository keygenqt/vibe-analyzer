//! Enriched project analysis structure.
//! Extends scanner ProjectAnalysis with LLM-generated summaries and enriched files.
use super::enriched_file_analysis::EnrichedFileAnalysis;
use crate::scanner::data::output::{
    license_info::LicenseInfo, project_stats::ProjectStats, readme_info::ReadmeInfo,
};
use serde::{Deserialize, Serialize};

/// Complete analysis results for a single project with LLM enrichment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnrichedProjectAnalysis {
    /// Absolute path to project root
    pub project: String,
    /// ISO 8601 timestamp of analysis
    pub indexed_at: String,
    /// README file content (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<ReadmeInfo>,
    /// LLM-generated project summary
    pub summary: String,
    /// Detected license information (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseInfo>,
    /// Aggregated project statistics
    pub stats: ProjectStats,
    /// Per-file analysis results
    pub files: Vec<EnrichedFileAnalysis>,
}
