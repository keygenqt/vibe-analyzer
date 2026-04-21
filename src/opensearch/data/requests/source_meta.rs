//! Source metadata for OpenSearch indexing.
//! Contains project-level information: summary, license, README, statistics.

use crate::analyzer::data::output::enriched_source_analysis::EnrichedSourceAnalysis;
use crate::opensearch::data::HasPath;
use crate::scanner::data::output::license_info::LicenseInfo;
use crate::scanner::data::output::readme_info::ReadmeInfo;
use crate::scanner::data::output::source_stats::SourceStats;
use serde::Deserialize;
use serde::Serialize;

/// Project metadata stored in vibe_meta_{hash} index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMeta {
    /// Absolute path to source root
    pub root: String,
    /// LLM-generated project summary
    pub summary: String,
    /// Detected README information (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<ReadmeInfo>,
    /// Detected license information (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseInfo>,
    /// Aggregated source statistics
    pub stats: SourceStats,
}

impl HasPath for SourceMeta {
    fn path(&self) -> &str {
        &self.root
    }
}

impl SourceMeta {
    /// Creates SourceMeta from EnrichedSourceAnalysis.
    fn new(root: &str, data: &EnrichedSourceAnalysis) -> Self {
        Self {
            root: root.to_string(),
            summary: data.summary.clone(),
            readme: data.readme.clone(),
            license: data.license.clone(),
            stats: data.stats.clone(),
        }
    }

    /// Converts a slice of EnrichedSourceAnalysis to a Vec of SourceMeta.
    pub fn from_enriched(sources: &[EnrichedSourceAnalysis]) -> Vec<Self> {
        sources.iter().map(|s| Self::new(&s.root, s)).collect()
    }
}
