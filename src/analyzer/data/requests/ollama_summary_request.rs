//! Request structure for generating project-level summary with LLM.
//! Contains project metadata, README, license information, and statistics.

use serde::Deserialize;
use serde::Serialize;

use crate::scanner::data::output::license_info::LicenseInfo;
use crate::scanner::data::output::project_analysis::ProjectAnalysis;
use crate::scanner::data::output::project_stats::ProjectStats;
use crate::scanner::data::output::readme_info::ReadmeInfo;

/// Request payload for project summary generation via Ollama.
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaSummaryRequest {
    /// Absolute path to project root
    pub project: String,
    /// README file content (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<ReadmeInfo>,
    /// Detected license information (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseInfo>,
    /// Aggregated project statistics
    pub stats: ProjectStats,
}

impl OllamaSummaryRequest {
    pub fn new(data: &ProjectAnalysis) -> Self {
        Self {
            project: data.project.clone(),
            readme: data.readme.clone(),
            license: data.license.clone(),
            stats: data.stats.clone(),
        }
    }
}
