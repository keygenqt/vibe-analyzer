//! C++ language AST parser
//!
//! This module provides tree-sitter based AST parsing for C++ files,
//! extracting functions, classes, structs, enums, includes, variables,
//! comments, and doc comments.

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{parser::AstData, traits::LanguageParser};

/// Parser for C++ language files
#[derive(Default)]
pub struct CppParser;

impl LanguageParser for CppParser {
    /// Parses C++ source code and extracts AST data
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        // API fix for tree-sitter 0.25.1
        parser.set_language(&tree_sitter_cpp::LANGUAGE.into())?;

        let tree = match parser.parse(content, None) {
            Some(tree) => tree,
            None => return Ok(AstData::default()),
        };

        let root_node = tree.root_node();
        let mut data = AstData::default();

        visit_node(&root_node, content, &mut data);
        clean_data(&mut data);

        Ok(data)
    }
}

/// Recursively traverses the AST and extracts relevant nodes
fn visit_node(node: &Node, content: &str, data: &mut AstData) {
    let kind = node.kind();

    match kind {
        // Comments
        "comment" => {
            if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                let cleaned = comment.trim().to_string();
                if !cleaned.is_empty() {
                    if cleaned.starts_with("/**") || cleaned.starts_with("/*!") {
                        data.doc_comments.push(cleaned);
                    } else {
                        data.comments.push(cleaned);
                    }
                }
            }
        }

        // Variables
        "declaration" => {
            if let Some(name) = extract_variable_name(node, content) {
                data.variables.push(name);
            }
        }

        // Functions
        "function_definition" => {
            if let Some(name) = extract_name(node, content) {
                data.functions.push(name);
            }
        }

        // Types
        "class_specifier" => {
            if let Some(name) = extract_name(node, content) {
                data.classes.push(name);
            }
        }
        "struct_specifier" => {
            if let Some(name) = extract_name(node, content) {
                data.structs.push(name);
            }
        }
        "enum_specifier" => {
            if let Some(name) = extract_name(node, content) {
                data.enums.push(name);
            }
        }

        // Includes (imports)
        "include_directive" => {
            if let Some(path) = extract_include_path(node, content) {
                data.imports.push(path);
            }
        }

        _ => {}
    }

    // Recursively process children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_node(&child, content, data);
    }
}

/// Extracts identifier/type name from a declaration node
fn extract_name(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        if kind == "identifier" || kind == "type_identifier" || kind == "field_identifier" {
            if let Ok(name) = child.utf8_text(content.as_bytes()) {
                if !name.is_empty() && name != "_" {
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}

/// Extracts variable name from a declaration node
fn extract_variable_name(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" {
            if let Ok(name) = child.utf8_text(content.as_bytes()) {
                let cleaned = name.trim().to_string();
                if !cleaned.is_empty() && cleaned != "_" {
                    return Some(cleaned);
                }
            }
        }
    }
    None
}

/// Extracts include path from #include directive
fn extract_include_path(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let cleaned = text
            .replace("#include ", "")
            .replace("<", "")
            .replace(">", "")
            .replace("\"", "")
            .trim()
            .to_string();
        if !cleaned.is_empty() && cleaned.len() < 200 {
            return Some(cleaned);
        }
    }
    None
}

/// Sorts and deduplicates all extracted data
fn clean_data(data: &mut AstData) {
    let fields = [
        &mut data.functions,
        &mut data.classes,
        &mut data.structs,
        &mut data.imports,
        &mut data.comments,
        &mut data.doc_comments,
        &mut data.variables,
        &mut data.traits,
        &mut data.enums,
    ];
    for field in fields {
        field.sort();
        field.dedup();
    }
}
