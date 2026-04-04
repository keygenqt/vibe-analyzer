use super::file_analysis::FileAnalysis;
use super::project_stats::ProjectStats;
use serde::{Deserialize, Serialize};

/// Complete analysis results for a single project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAnalysis {
    /// Absolute path to the project root directory
    pub project: String,
    /// ISO 8601 timestamp of when the analysis was performed
    pub indexed_at: String,
    /// Aggregated statistics across all files in the project
    pub stats: ProjectStats,
    /// Detailed analysis for each file in the project
    pub files: Vec<FileAnalysis>,
}
