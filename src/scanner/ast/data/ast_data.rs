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
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub header_comments: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub imports: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub variables: Vec<AstDataVariable>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub functions: Vec<AstDataFunction>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub classes: Vec<AstDataClass>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub structs: Vec<AstDataStruct>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub enums: Vec<AstDataEnum>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub interfaces: Vec<AstDataInterface>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub frontmatter: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub headings: Vec<AstHeading>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub links: Vec<AstLink>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub code_blocks: Vec<String>,
}
