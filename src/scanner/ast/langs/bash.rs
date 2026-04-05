//! Bash/Shell language AST parser
//!
//! This module provides tree-sitter based AST parsing for Bash/Shell scripts,
//! extracting functions, variables, source imports, comments, and doc comments.

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{parser::AstData, traits::LanguageParser};

/// Parser for Bash/Shell language files
#[derive(Default)]
pub struct BashParser;

impl LanguageParser for BashParser {
    /// Parses Bash/Shell source code and extracts AST data
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_bash::LANGUAGE.into())?;

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
                    // Multi-line comments starting with ## or ### are treated as doc comments
                    if cleaned.starts_with("##") || cleaned.starts_with("###") {
                        data.doc_comments.push(cleaned);
                    } else {
                        data.comments.push(cleaned);
                    }
                }
            }
        }

        // Variables
        "variable_assignment" => {
            if let Some(name) = extract_variable_name(node, content) {
                data.variables.push(name);
            }
        }

        // Functions
        "function_definition" => {
            if let Some(name) = extract_function_name(node, content) {
                data.functions.push(name);
            }
        }

        // Source/import commands (like `source ./file.sh` or `. ./file.sh`)
        "source_command" => {
            if let Some(path) = extract_source_path(node, content) {
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

/// Extracts variable name from a variable assignment node
fn extract_variable_name(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "variable_name" {
            if let Ok(name) = child.utf8_text(content.as_bytes()) {
                let cleaned = name.trim().to_string();
                if !cleaned.is_empty() && !cleaned.starts_with('#') {
                    return Some(cleaned);
                }
            }
        }
    }
    None
}

/// Extracts function name from a function definition node
fn extract_function_name(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        if kind == "word" || kind == "identifier" || kind == "name" {
            if let Ok(name) = child.utf8_text(content.as_bytes()) {
                if !name.is_empty() && name != "function" && !name.contains(' ') {
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}

/// Extracts source file path from a source command
fn extract_source_path(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        if kind == "word" || kind == "string" || kind == "raw_string" {
            if let Ok(path) = child.utf8_text(content.as_bytes()) {
                let cleaned = path
                    .replace("source ", "")
                    .replace(".", "")
                    .trim()
                    .to_string();
                if !cleaned.is_empty() && !cleaned.starts_with('#') {
                    return Some(cleaned);
                }
            }
        }
    }
    None
}

/// Sorts and deduplicates all extracted data
fn clean_data(data: &mut AstData) {
    let fields = [
        &mut data.functions,
        &mut data.imports,
        &mut data.comments,
        &mut data.doc_comments,
        &mut data.variables,
    ];
    for field in fields {
        field.sort();
        field.dedup();
    }
}
