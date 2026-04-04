use super::project_analysis::ProjectAnalysis;
use serde::{Deserialize, Serialize};

/// Container for multiple project analyses, used for batch JSON export
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MultiProjectAnalysis {
    /// ISO 8601 timestamp of when the batch export was created
    pub indexed_at: String,
    /// Collection of individual project analyses
    pub projects: Vec<ProjectAnalysis>,
}
