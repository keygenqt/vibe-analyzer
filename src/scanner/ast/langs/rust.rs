//! Rust language AST parser
//!
//! This module provides tree-sitter based AST parsing for Rust files,
//! extracting functions, structs, enums, traits, use declarations (imports),
//! variables, comments, and doc comments (///, //!, /**).

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{parser::AstData, traits::LanguageParser};

/// Parser for Rust language files
#[derive(Default)]
pub struct RustParser;

impl LanguageParser for RustParser {
    /// Parses Rust source code and extracts AST data
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_rust::LANGUAGE.into())?;

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
        "line_comment" | "block_comment" => {
            if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                let cleaned = comment.trim().to_string();
                if !cleaned.is_empty() {
                    if cleaned.starts_with("///")
                        || cleaned.starts_with("//!")
                        || cleaned.starts_with("/**")
                        || cleaned.starts_with("/*!")
                    {
                        data.doc_comments.push(cleaned);
                    } else {
                        data.comments.push(cleaned);
                    }
                }
            }
        }

        // Variables
        "let_declaration" => {
            if let Some(name) = extract_variable_name(node, content) {
                data.variables.push(name);
            }
        }

        // Functions and methods
        "function_item" | "method_definition" => {
            if let Some(name) = extract_name(node, content) {
                data.functions.push(name);
            }
        }

        // Structs
        "struct_item" => {
            if let Some(name) = extract_name(node, content) {
                data.structs.push(name);
            }
        }

        // Enums
        "enum_item" => {
            if let Some(name) = extract_name(node, content) {
                data.enums.push(name);
            }
        }

        // Traits
        "trait_item" => {
            if let Some(name) = extract_name(node, content) {
                data.traits.push(name);
            }
        }

        // Imports
        "use_declaration" => {
            if let Some(path) = extract_import_path(node, content) {
                data.imports.push(path);
            }
        }

        // Ignored items (implementation blocks)
        "impl_item" => {}

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
        if kind == "identifier" || kind == "type_identifier" {
            if let Ok(name) = child.utf8_text(content.as_bytes()) {
                let cleaned = name.trim();
                if !cleaned.is_empty() && cleaned != "self" && cleaned != "Self" && cleaned != "_" {
                    return Some(cleaned.to_string());
                }
            }
        }
    }
    None
}

/// Extracts variable name from a let declaration
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

/// Extracts import path from use declaration
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
