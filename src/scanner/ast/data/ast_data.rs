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

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,
}

impl AstData {
    /// Consumes self and returns AstData with tags populated.
    pub fn with_tags(mut self) -> Self {
        self.gen_tags();
        self
    }

    /// Generates multilingual tags for AST structure search.
    /// Adds English, Russian, and Chinese variants for each present AST field.
    fn gen_tags(&mut self) {
        // English tags
        if !self.header_comments.is_empty() {
            self.tags.push("header_comments".to_string());
        }
        if !self.imports.is_empty() {
            self.tags.push("imports".to_string());
            self.tags.push("импорты".to_string());
            self.tags.push("导入".to_string());
        }
        if !self.variables.is_empty() {
            self.tags.push("variables".to_string());
            self.tags.push("переменные".to_string());
            self.tags.push("变量".to_string());
        }
        if !self.functions.is_empty() {
            self.tags.push("functions".to_string());
            self.tags.push("функции".to_string());
            self.tags.push("函数".to_string());
        }
        if !self.classes.is_empty() {
            self.tags.push("classes".to_string());
            self.tags.push("классы".to_string());
            self.tags.push("类".to_string());
        }
        if !self.structs.is_empty() {
            self.tags.push("structs".to_string());
            self.tags.push("структуры".to_string());
            self.tags.push("结构体".to_string());
        }
        if !self.enums.is_empty() {
            self.tags.push("enums".to_string());
            self.tags.push("перечисления".to_string());
            self.tags.push("枚举".to_string());
        }
        if !self.interfaces.is_empty() {
            self.tags.push("interfaces".to_string());
            self.tags.push("интерфейсы".to_string());
            self.tags.push("接口".to_string());
        }
        if self.frontmatter.is_some() {
            self.tags.push("frontmatter".to_string());
        }
        if !self.headings.is_empty() {
            self.tags.push("headings".to_string());
            self.tags.push("заголовки".to_string());
            self.tags.push("标题".to_string());
        }
        if !self.links.is_empty() {
            self.tags.push("links".to_string());
            self.tags.push("ссылки".to_string());
            self.tags.push("链接".to_string());
        }
        if !self.code_blocks.is_empty() {
            self.tags.push("code_blocks".to_string());
            self.tags.push("блоки_кода".to_string());
            self.tags.push("代码块".to_string());
        }
    }
}
