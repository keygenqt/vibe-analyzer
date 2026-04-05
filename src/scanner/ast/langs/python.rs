//! Python language AST parser
//!
//! This module provides tree-sitter based AST parsing for Python files,
//! extracting functions, classes, imports, variables, comments, and doc comments
//! (including triple-quoted docstrings).

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{parser::AstData, traits::LanguageParser};

/// Parser for Python language files
#[derive(Default)]
pub struct PythonParser;

impl LanguageParser for PythonParser {
    /// Parses Python source code and extracts AST data
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_python::LANGUAGE.into())?;

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
        // Regular comments
        "comment" => {
            if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                let cleaned = comment.trim().to_string();
                if !cleaned.is_empty() {
                    data.comments.push(cleaned);
                }
            }
        }

        // Docstrings (triple-quoted strings as expression statements)
        "expression_statement" => {
            if let Some(string_node) = node.child(0) {
                if string_node.kind() == "string" {
                    if let Ok(text) = string_node.utf8_text(content.as_bytes()) {
                        let cleaned = text.trim();
                        if cleaned.starts_with("\"\"\"") || cleaned.starts_with("'''") {
                            data.doc_comments.push(cleaned.to_string());
                        }
                    }
                }
            }
        }

        // Variables
        "assignment" => {
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

        // Classes
        "class_definition" => {
            if let Some(name) = extract_name(node, content) {
                data.classes.push(name);
            }
        }

        // Imports
        "import_statement" | "import_from_statement" => {
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
                if !cleaned.is_empty() && cleaned != "_" {
                    return Some(cleaned.to_string());
                }
            }
        }
    }
    None
}

/// Extracts variable name from an assignment node
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

/// Extracts import path from import statement
fn extract_import_path(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let cleaned = text
            .replace("import ", "")
            .replace("from ", "")
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
