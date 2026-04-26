//! Source metadata for OpenSearch indexing.
//! Contains project-level information: summary, license, README, statistics.

use crate::analyzer::data::output::analyzer_project_output::AnalyzerProjectOutput;
use crate::opensearch::data::HasPath;
use crate::scanner::data::output::scan_license_output::ScanLicenseOutput;
use crate::scanner::data::output::scan_project_stats_output::ScanProjectStatsOutput;
use crate::scanner::data::output::scan_readme_output::ScanReadmeOutput;
use serde::Deserialize;
use serde::Serialize;

/// Project metadata stored in vibe_meta_{hash} index.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "ProjectMeta")]
pub struct OpensearchProjectMetaRequest {
    /// Absolute path to source root
    pub root: String,
    /// LLM-generated project summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Detected README information (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<ScanReadmeOutput>,
    /// Detected license information (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<ScanLicenseOutput>,
    /// Aggregated source statistics
    pub stats: ScanProjectStatsOutput,
}

impl HasPath for OpensearchProjectMetaRequest {
    fn path(&self) -> &str {
        &self.root
    }
}

impl OpensearchProjectMetaRequest {
    /// Creates SourceMeta from EnrichedSourceAnalysis.
    fn new(root: &str, data: &AnalyzerProjectOutput) -> Self {
        Self {
            root: root.to_string(),
            summary: data.summary.clone(),
            readme: data.readme.clone(),
            license: data.license.clone(),
            stats: data.stats.clone(),
        }
    }

    /// Converts a slice of EnrichedSourceAnalysis to a Vec of SourceMeta.
    pub fn from_enriched(sources: &[AnalyzerProjectOutput]) -> Vec<Self> {
        sources.iter().map(|s| Self::new(&s.root, s)).collect()
    }
}
