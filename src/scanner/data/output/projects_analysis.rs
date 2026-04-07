//! Multi-project batch export container.
//! Wraps multiple project analyses into a single JSON export with a timestamp.

use super::project_analysis::ProjectAnalysis;
use serde::Deserialize;
use serde::Serialize;

/// Container for multiple project analyses (batch JSON export)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ProjectsAnalysis {
    /// ISO 8601 timestamp of the batch export
    pub indexed_at: String,
    /// Collection of individual project analyses
    pub projects: Vec<ProjectAnalysis>,
}
