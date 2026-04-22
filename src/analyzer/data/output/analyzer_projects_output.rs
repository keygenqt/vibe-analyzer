//! Multi-source enriched analysis container.
//! Wraps multiple enriched source analyses for batch export.

use serde::Deserialize;
use serde::Serialize;

use crate::analyzer::data::output::analyzer_project_output::AnalyzerProjectOutput;

/// Container for multiple enriched source analyses exported to JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "ProjectsEnriched")]
pub struct AnalyzerProjectsOutput {
    /// When the initial scan was performed
    pub indexed_at: String,
    /// List of enriched source analyses
    pub sources: Vec<AnalyzerProjectOutput>,
}
