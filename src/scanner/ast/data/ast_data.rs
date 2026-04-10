//! Main AST data structure.
//! Aggregates all extracted code elements (functions, classes, imports, structs, enums, interfaces, variables, and comments) from a parsed file.

use std::collections::HashMap;

use super::ast_data_class::AstDataClass;
use super::ast_data_enum::AstDataEnum;
use super::ast_data_function::AstDataFunction;
use super::ast_data_interface::AstDataInterface;
use super::ast_data_struct::AstDataStruct;
use super::ast_data_variable::AstDataVariable;
use crate::scanner::ast::data::ast_data_heading::AstHeading;
use crate::scanner::ast::data::ast_data_links::AstLink;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AstData {
    /// Top-level documentation comments at the beginning of the file
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub header_comments: Vec<String>,

    /// Import/use/include statements
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<String>,

    /// Variables with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<AstDataVariable>,

    /// Functions with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<AstDataFunction>,

    /// Classes with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub classes: Vec<AstDataClass>,

    /// Structs with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub structs: Vec<AstDataStruct>,

    /// Enums with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enums: Vec<AstDataEnum>,

    /// Interfaces/Protocols/Traits with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub interfaces: Vec<AstDataInterface>,

    /// Markdown frontmatter metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontmatter: Option<HashMap<String, serde_json::Value>>,

    /// Markdown headings
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub headings: Vec<AstHeading>,

    /// Markdown links
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<AstLink>,

    /// Markdown code block languages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub code_blocks: Vec<String>,
}
