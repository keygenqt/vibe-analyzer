//! Java language AST parser
//!
//! This module provides tree-sitter based AST parsing for Java files,
//! extracting methods (functions), classes, interfaces (as traits), enums,
//! imports, variables, comments, and doc comments.

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{parser::AstData, traits::LanguageParser};

/// Parser for Java language files
#[derive(Default)]
pub struct JavaParser;

impl LanguageParser for JavaParser {
    /// Parses Java source code and extracts AST data
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_java::LANGUAGE.into())?;

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
                    if cleaned.starts_with("/**") {
                        data.doc_comments.push(cleaned);
                    } else {
                        data.comments.push(cleaned);
                    }
                }
            }
        }

        // Variables (fields and local variables)
        "field_declaration" | "local_variable_declaration" => {
            if let Some(name) = extract_variable_name(node, content) {
                data.variables.push(name);
            }
        }

        // Methods
        "method_declaration" => {
            if let Some(name) = extract_name(node, content) {
                data.functions.push(name);
            }
        }

        // Types
        "class_declaration" => {
            if let Some(name) = extract_name(node, content) {
                data.classes.push(name);
            }
        }
        "interface_declaration" => {
            if let Some(name) = extract_name(node, content) {
                data.traits.push(name);
            }
        }
        "enum_declaration" => {
            if let Some(name) = extract_name(node, content) {
                data.enums.push(name);
            }
        }

        // Imports
        "import_declaration" => {
            if let Some(path) = extract_import_path(node, content) {
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

/// Extracts identifier name from a declaration node
fn extract_name(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" {
            if let Ok(name) = child.utf8_text(content.as_bytes()) {
                let cleaned = name.trim();
                if !cleaned.is_empty() {
                    return Some(cleaned.to_string());
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
                let cleaned = name.trim();
                if !cleaned.is_empty() && cleaned != "_" {
                    return Some(cleaned.to_string());
                }
            }
        }
        // Handle variable_declarator nested nodes
        if child.kind() == "variable_declarator" {
            if let Some(name) = extract_variable_name(&child, content) {
                return Some(name);
            }
        }
    }
    None
}

/// Extracts import path from import declaration
fn extract_import_path(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let cleaned = text
            .replace("import ", "")
            .replace("static ", "")
            .replace(";", "")
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
