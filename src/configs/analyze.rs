//! Analyze configuration.
//! Controls LLM enrichment batching parameters.

use serde::Deserialize;
use serde::Serialize;

/// Batch settings for LLM-powered code enrichment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeConfig {
    /// Maximum characters per request batch.
    pub max_chunk_chars: usize,
    /// Maximum files per request batch.
    pub max_chunk_files: usize,
}

impl Default for AnalyzeConfig {
    /// Sensible defaults for local LLM processing.
    fn default() -> Self {
        Self {
            max_chunk_chars: 3_000,
            max_chunk_files: 3,
        }
    }
}
