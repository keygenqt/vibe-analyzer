//! Analyze configuration.
//! Controls LLM enrichment batching parameters.

use serde::Deserialize;
use serde::Serialize;

use crate::analyzer::AnalyzeMode;

/// Batch settings for LLM-powered code enrichment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeConfig {
    /// Maximum characters per request batch.
    pub max_chunk_chars: usize,
    /// Maximum files per request batch.
    pub max_chunk_files: usize,
    /// @todo
    pub include: AnalyzeIncludeConfig,
}

impl Default for AnalyzeConfig {
    /// Sensible defaults for local LLM processing.
    fn default() -> Self {
        Self {
            max_chunk_chars: 3_000,
            max_chunk_files: 3,
            include: AnalyzeIncludeConfig::default(),
        }
    }
}

/// Enrichment features to include during analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeIncludeConfig {
    /// Generate summary and tags for the file.
    pub meta: bool,
    /// Detect technical debt: TODO, magic numbers, commented-out code.
    pub debt: bool,
    /// Suggest refactoring, naming, and test improvements.
    pub advice: bool,
    /// Find bugs, spelling errors, and potential issues.
    pub errors: bool,
}

impl Default for AnalyzeIncludeConfig {
    /// Scan everything except imports by default.
    fn default() -> Self {
        Self {
            meta: false,
            debt: false,
            errors: false,
            advice: false,
        }
    }
}

impl AnalyzeIncludeConfig {
    /// Returns list of enabled enrichment modes. Ast is always included.
    pub fn list(&self) -> Vec<AnalyzeMode> {
        let mut modes = vec![AnalyzeMode::Ast];
        if self.meta {
            modes.push(AnalyzeMode::Meta);
        }
        if self.debt {
            modes.push(AnalyzeMode::Debt);
        }
        if self.errors {
            modes.push(AnalyzeMode::Errors);
        }
        if self.advice {
            modes.push(AnalyzeMode::Advice);
        }
        modes
    }
}
