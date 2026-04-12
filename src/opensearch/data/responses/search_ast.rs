//! AST search response model.
//! Contains file path and matching AST elements.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    opensearch::data::responses::search_hit::SearchHit,
    scanner::ast::data::{
        ast_data_class::AstDataClass, ast_data_enum::AstDataEnum,
        ast_data_function::AstDataFunction, ast_data_heading::AstHeading,
        ast_data_interface::AstDataInterface, ast_data_links::AstLink,
        ast_data_struct::AstDataStruct, ast_data_variable::AstDataVariable,
    },
};

/// Response from AST search query containing matched file and filtered AST data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchAst {
    /// Relative file path where matches were found.
    pub path: String,
    /// Top-level documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub header_comments: Vec<String>,
    /// Import/use/include statements matching the search pattern.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<String>,
    /// Variable definitions matching the search pattern.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<AstDataVariable>,
    /// Function signatures matching the search pattern.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<AstDataFunction>,
    /// Class signatures matching the search pattern.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub classes: Vec<AstDataClass>,
    /// Struct signatures matching the search pattern.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub structs: Vec<AstDataStruct>,
    /// Enum signatures matching the search pattern.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enums: Vec<AstDataEnum>,
    /// Interface/trait signatures matching the search pattern.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub interfaces: Vec<AstDataInterface>,
    /// Markdown frontmatter metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontmatter: Option<HashMap<String, String>>,
    /// Markdown headings matching the search pattern.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub headings: Vec<AstHeading>,
    /// Markdown links matching the search pattern.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<AstLink>,
    /// Code block languages found in markdown.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub code_blocks: Vec<String>,
    /// Base metadata included in every search result.
    pub hit: SearchHit,
}
