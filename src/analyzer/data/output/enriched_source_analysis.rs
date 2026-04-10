//! Enriched source analysis structure.
//! Extends scanner SourceAnalysis with LLM-generated summaries and enriched files.
use super::enriched_file_analysis::EnrichedFileAnalysis;
use crate::scanner::data::output::license_info::LicenseInfo;
use crate::scanner::data::output::source_stats::SourceStats;
use crate::scanner::data::output::readme_info::ReadmeInfo;
use serde::Deserialize;
use serde::Serialize;

/// Complete analysis results for a single source with LLM enrichment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnrichedSourceAnalysis {
    /// Absolute path to source root
    pub source: String,
    /// ISO 8601 timestamp of analysis
    pub indexed_at: String,
    /// README file content (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<ReadmeInfo>,
    /// LLM-generated source summary
    pub summary: String,
    /// Detected license information (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseInfo>,
    /// Aggregated source statistics
    pub stats: SourceStats,
    /// Per-file analysis results
    pub files: Vec<EnrichedFileAnalysis>,
}
