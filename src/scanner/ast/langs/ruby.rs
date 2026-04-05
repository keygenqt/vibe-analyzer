//! Ruby language AST parser
//!
//! This module provides tree-sitter based AST parsing for Ruby files,
//! extracting methods (functions), classes, modules, requires (imports),
//! variables, and comments.

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{parser::AstData, traits::LanguageParser};

/// Parser for Ruby language files
#[derive(Default)]
pub struct RubyParser;

impl LanguageParser for RubyParser {
    /// Parses Ruby source code and extracts AST data
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_ruby::LANGUAGE.into())?;

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
                    data.comments.push(cleaned);
                }
            }
        }

        // Variables
        "assignment" | "operator_assignment" => {
            if let Some(name) = extract_variable_name(node, content) {
                data.variables.push(name);
            }
        }

        // Methods (functions)
        "method" | "singleton_method" => {
            if let Some(name) = extract_name(node, content) {
                data.functions.push(name);
            }
        }

        // Classes and modules
        "class" | "module" => {
            if let Some(name) = extract_name(node, content) {
                data.classes.push(name);
            }
        }

        // Requires (imports)
        "require" | "require_relative" => {
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

/// Extracts name from method, class, or module nodes
fn extract_name(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        if kind == "constant" || kind == "identifier" || kind == "symbol" {
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

/// Extracts variable name from assignment nodes
fn extract_variable_name(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        if kind == "identifier" || kind == "instance_variable" || kind == "constant" {
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

/// Extracts import path from require statement
fn extract_import_path(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let cleaned = text
            .replace("require ", "")
            .replace("require_relative ", "")
            .replace("\"", "")
            .replace("'", "")
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
