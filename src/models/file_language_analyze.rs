//! File language analyze model.
//! Stores LLM enrichment results: summary and search tags.

use serde::Deserialize;
use serde::Serialize;

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
}

/// Format implementation for file language analyze model.
impl TraitFormatModel for FileLanguageAnalyzeModel {
    fn format_ast(&self) -> Self {
        Self {
            summary: None,
            tags: Vec::new(),
        }
    }

    fn format_analyze(&self) -> Self {
        Self {
            summary: self.summary.clone(),
            tags: self.tags.clone(),
        }
    }

    fn format_index(&self) -> Self {
        self.format_analyze()
    }

    fn format_tool(&self) -> Self {
        self.format_ast()
    }
}
