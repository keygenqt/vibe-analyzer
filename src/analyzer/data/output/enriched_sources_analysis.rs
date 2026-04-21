//! Multi-source enriched analysis container.
//! Wraps multiple enriched source analyses for batch export.

use super::enriched_source_analysis::EnrichedSourceAnalysis;
use serde::Deserialize;
use serde::Serialize;

/// Container for multiple enriched source analyses exported to JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichedSourcesAnalysis {
    /// When the initial scan was performed
    pub indexed_at: String,
    /// List of enriched source analyses
    pub sources: Vec<EnrichedSourceAnalysis>,
}
