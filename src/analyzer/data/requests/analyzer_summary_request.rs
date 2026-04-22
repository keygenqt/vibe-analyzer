//! Request structure for generating source-level summary with LLM.
//! Contains source metadata, README, license information, and statistics.

use serde::Deserialize;
use serde::Serialize;

use crate::scanner::data::output::scan_license_output::ScanLicenseOutput;
use crate::scanner::data::output::scan_project_analysis_output::ScanProjectAnalysisOutput;
use crate::scanner::data::output::scan_project_stats_output::ScanProjectStatsOutput;
use crate::scanner::data::output::scan_readme_output::ScanReadmeOutput;

/// Request payload for source summary generation via Ollama.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Summary")]
pub struct AnalyzerSummaryRequest {
    /// Absolute path to source root
    pub root: String,
    /// README file content (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<ScanReadmeOutput>,
    /// Detected license information (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<ScanLicenseOutput>,
    /// Aggregated source statistics
    pub stats: ScanProjectStatsOutput,
}

impl AnalyzerSummaryRequest {
    pub fn new(data: &ScanProjectAnalysisOutput) -> Self {
        Self {
            root: data.root.clone(),
            readme: data.readme.clone(),
            license: data.license.clone(),
            stats: data.stats.clone(),
        }
    }
}
