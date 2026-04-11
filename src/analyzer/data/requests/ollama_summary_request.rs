//! Request structure for generating source-level summary with LLM.
//! Contains source metadata, README, license information, and statistics.

use serde::Deserialize;
use serde::Serialize;

use crate::scanner::data::output::license_info::LicenseInfo;
use crate::scanner::data::output::readme_info::ReadmeInfo;
use crate::scanner::data::output::source_analysis::SourceAnalysis;
use crate::scanner::data::output::source_stats::SourceStats;

/// Request payload for source summary generation via Ollama.
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaSummaryRequest {
    /// Absolute path to source root
    pub root: String,
    /// README file content (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<ReadmeInfo>,
    /// Detected license information (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseInfo>,
    /// Aggregated source statistics
    pub stats: SourceStats,
}

impl OllamaSummaryRequest {
    pub fn new(data: &SourceAnalysis) -> Self {
        Self {
            root: data.root.clone(),
            readme: data.readme.clone(),
            license: data.license.clone(),
            stats: data.stats.clone(),
        }
    }
}
