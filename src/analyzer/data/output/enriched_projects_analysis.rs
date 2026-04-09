//! Multi-project enriched analysis container.
//! Wraps multiple enriched project analyses for batch export.

use super::enriched_project_analysis::EnrichedProjectAnalysis;
use serde::Deserialize;
use serde::Serialize;

/// Container for multiple enriched project analyses exported to JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnrichedProjectsAnalysis {
    /// When the initial scan was performed
    pub indexed_at: String,
    /// List of enriched project analyses
    pub projects: Vec<EnrichedProjectAnalysis>,
}
