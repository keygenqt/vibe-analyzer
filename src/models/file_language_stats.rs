//! File language code model.
//! Code structures extracted from source files.

use serde::Deserialize;
use serde::Serialize;

use crate::analyzer::AnalyzeMode;
use crate::models::trait_format_model::TraitFormatModel;

/// Stats analysis results from AST parsing (non-LLM).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileLanguageStatsModel {
    /// Code markers found during parsing (TODO, FIXME, HACK, etc.)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub markers: Vec<String>,
    /// Warnings found during parsing (unwrap, panic, bare except, etc.)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<String>,
}

impl TraitFormatModel for FileLanguageStatsModel {
    fn format_ast(&self) -> Self {
        self.clone()
    }

    fn format_analyze(&self) -> Self {
        self.clone()
    }

    fn format_index(&self) -> Self {
        Self::default()
    }

    fn format_tool(&self) -> Self {
        Self::default()
    }

    fn format_batch(&self, mode: AnalyzeMode) -> Self {
        match mode {
            AnalyzeMode::Debt => Self {
                markers: self.markers.clone(),
                warnings: vec![],
            },
            AnalyzeMode::Errors => Self {
                markers: vec![],
                warnings: self.warnings.clone(),
            },
            _ => Self::default(),
        }
    }
}
