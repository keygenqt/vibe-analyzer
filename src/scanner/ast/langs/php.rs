//! PHP language AST parser
//!
//! This module provides tree-sitter based AST parsing for PHP files,
//! extracting functions, methods, classes, interfaces, traits, imports (use statements),
//! variables, comments, and doc comments.

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{parser::AstData, traits::LanguageParser};

/// Parser for PHP language files
#[derive(Default)]
pub struct PhpParser;

impl LanguageParser for PhpParser {
    /// Parses PHP source code and extracts AST data
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_php::LANGUAGE_PHP.into())?;

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

        // Variables
        "variable_declaration" | "assignment_expression" => {
            if let Some(name) = extract_variable_name(node, content) {
                data.variables.push(name);
            }
        }

        // Functions and methods
        "function_definition" | "method_declaration" => {
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
        "interface_declaration" | "trait_declaration" => {
            if let Some(name) = extract_name(node, content) {
                data.traits.push(name);
            }
        }

        // Imports (use statements)
        "namespace_use_declaration" => {
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

/// Extracts name from function, class, interface, or trait declaration
fn extract_name(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        if kind == "name" || kind == "identifier" {
            if let Ok(name) = child.utf8_text(content.as_bytes()) {
                if !name.is_empty() && name != "_" {
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}

/// Extracts variable name, stripping the leading `$` character
fn extract_variable_name(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "variable_name" {
            if let Ok(name) = child.utf8_text(content.as_bytes()) {
                let cleaned = name.trim().to_string();
                if !cleaned.is_empty() && cleaned != "_" {
                    return Some(cleaned.trim_start_matches('$').to_string());
                }
            }
        }
    }
    None
}

/// Extracts import path from use statement
fn extract_import_path(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let cleaned = text.replace("use ", "").replace(";", "").trim().to_string();
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
