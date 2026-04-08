//! Enriched file analysis structure.
//! Extends scanner FileAnalysis with LLM-generated metadata.

use crate::scanner::data::output::file_analysis::FileAnalysis;
use serde::{Deserialize, Serialize};

/// File analysis enriched with LLM-generated description and search tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnrichedFileAnalysis {
    /// LLM-generated description of file purpose and main functionality
    pub description: Option<String>,
    /// Search tags for improved discoverability
    pub tags: Option<Vec<String>>,
    /// Original file analysis data from scanner
    pub base: FileAnalysis,
}
