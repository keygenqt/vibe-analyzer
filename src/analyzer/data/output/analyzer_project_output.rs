//! Enriched source analysis structure.
//! Extends scanner SourceAnalysis with LLM-generated summaries and enriched files.

use crate::analyzer::data::output::analyzer_file_output::AnalyzerFileOutput;
use crate::scanner::data::output::scan_license_output::ScanLicenseOutput;
use crate::scanner::data::output::scan_project_stats_output::ScanProjectStatsOutput;
use crate::scanner::data::output::scan_readme_output::ScanReadmeOutput;
use serde::Deserialize;
use serde::Serialize;

/// Complete analysis results for a single source with LLM enrichment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "ProjectEnriched")]
pub struct AnalyzerProjectOutput {
    /// Absolute path to source root
    pub root: String,
    /// ISO 8601 timestamp of analysis
    pub indexed_at: String,
    /// README file content (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<ScanReadmeOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// LLM-generated source summary
    pub summary: Option<String>,
    /// Detected license information (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<ScanLicenseOutput>,
    /// Aggregated source statistics
    pub stats: ScanProjectStatsOutput,
    /// Per-file analysis results
    pub files: Vec<AnalyzerFileOutput>,
}
