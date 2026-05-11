//! Response structure for the search_by_code_classes tool.

use std::convert::TryFrom;

use anyhow::Result;
use serde::Serialize;

use crate::mcp::responses::McpResponse;
use crate::models::file_models::FileModels;
use crate::models::lang_class::LangClassModel;
use crate::models::lang_interface::LangInterfaceModel;
use crate::models::lang_struct::LangStructModel;
use crate::models::trait_base_model::TraitBaseModel;
use crate::models::trait_file_model::TraitFileModel;
use crate::models::trait_format_model::TraitFormatModel;

/// Response from search_by_code_classes tool.
#[derive(Debug, Serialize)]
pub struct SearchByCodeClassesResponse {
    /// Search query that was used.
    pub query: String,
    /// Found files with matching classes, structs, or interfaces.
    pub results: Vec<SearchByCodeClassesMetaResponse>,
}

/// File metadata with matching classes, structs, and interfaces.
#[derive(Debug, Serialize)]
pub struct SearchByCodeClassesMetaResponse {
    /// Relative file path.
    pub path: String,
    /// Programming language of the file.
    pub language: String,
    /// Module-level documentation comments.
    pub header_comments: Vec<String>,
    /// Matching class declarations.
    pub classes: Vec<LangClassModel>,
    /// Matching struct declarations.
    pub structs: Vec<LangStructModel>,
    /// Matching interface declarations.
    pub interfaces: Vec<LangInterfaceModel>,
}

/// Converts file model to search result, filtering non-language files.
impl TryFrom<&FileModels> for SearchByCodeClassesMetaResponse {
    type Error = anyhow::Error;

    fn try_from(model: &FileModels) -> Result<Self> {
        match model {
            FileModels::Language(m) => Ok(SearchByCodeClassesMetaResponse {
                path: m.base_model().path().to_string(),
                language: m.language.clone(),
                header_comments: m.code.header_comments.clone(),
                classes: m.code.classes.iter().map(|c| c.format_tool()).collect(),
                structs: m.code.structs.iter().map(|s| s.format_tool()).collect(),
                interfaces: m.code.interfaces.iter().map(|i| i.format_tool()).collect(),
            }),
            _ => anyhow::bail!("AST not available for this file type"),
        }
    }
}

impl McpResponse for SearchByCodeClassesResponse {
    /// Returns a formatted tool result with JSON data.
    fn format(&self) -> String {
        format!(
            "[{} type(s) found for '{}']\nUse this data to respond to the user.\n\n{}",
            self.results.len(),
            self.query,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
