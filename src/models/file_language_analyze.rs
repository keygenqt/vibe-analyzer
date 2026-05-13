//! File language analyze model.
//! Stores LLM enrichment results: summary and search tags.

use serde::Deserialize;
use serde::Serialize;

use crate::analyzer::AnalyzeMode;
use crate::models::trait_format_model::TraitFormatModel;

/// LLM enrichment — summary, search tags, and element analysis.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileLanguageAnalyzeModel {
    /// LLM-generated file summary.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub summary: Option<String>,
    /// Search tags (structural + LLM-generated).
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,
    /// Technical debt: TODO, magic numbers, commented-out code.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub debt: Vec<String>,
    /// Refactoring and test suggestions.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub advice: Vec<String>,
    /// Bugs and spelling errors.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub errors: Vec<String>,
}

impl FileLanguageAnalyzeModel {
    /// Merge enrichment data from another model
    pub fn merge(&mut self, other: &Self) {
        if let Some(summary) = &other.summary {
            self.summary = Some(summary.clone());
        }
        if !other.tags.is_empty() {
            self.tags = other.tags.clone();
        }
        if !other.debt.is_empty() {
            self.debt = other.debt.clone();
        }
        if !other.advice.is_empty() {
            self.advice = other.advice.clone();
        }
        if !other.errors.is_empty() {
            self.errors = other.errors.clone();
        }
    }
}

/// Format implementation for file language analyze model.
impl TraitFormatModel for FileLanguageAnalyzeModel {
    fn format_ast(&self) -> Self {
        Self::default()
    }

    fn format_analyze(&self) -> Self {
        Self {
            summary: self.summary.clone(),
            tags: self.tags.clone(),
            debt: self.debt.clone(),
            advice: self.advice.clone(),
            errors: self.errors.clone(),
        }
    }

    fn format_index(&self) -> Self {
        Self {
            summary: self.summary.clone(),
            tags: self.tags.clone(),
            debt: vec![],
            advice: vec![],
            errors: vec![],
        }
    }

    fn format_tool(&self) -> Self {
        Self::default()
    }

    fn format_batch(&self, _mode: AnalyzeMode) -> Self {
        Self::default()
    }
}
