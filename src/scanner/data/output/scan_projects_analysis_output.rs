//! Multi-source batch export container.
//! Wraps multiple source analyses into a single JSON export with a timestamp.

use super::scan_project_analysis_output::ScanProjectAnalysisOutput;
use serde::Deserialize;
use serde::Serialize;

/// Container for multiple source analyses (batch JSON export)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "ProjectsAnalysis")]
pub struct ScanProjectsAnalysisOutput {
    /// ISO 8601 timestamp of the batch export
    pub indexed_at: String,
    /// Collection of individual source analyses
    pub sources: Vec<ScanProjectAnalysisOutput>,
}
