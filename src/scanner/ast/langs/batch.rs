//! Batch script language AST parser
//!
//! This module provides tree-sitter based AST parsing for Windows Batch files (.bat, .cmd),
//! extracting labels (as functions), variables, commands (as imports), and comments.

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{parser::AstData, traits::LanguageParser};

/// Parser for Windows Batch language files
#[derive(Default)]
pub struct BatchParser;

impl LanguageParser for BatchParser {
    /// Parses Batch source code and extracts AST data
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_batch::LANGUAGE.into())?;

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
        // Variables (e.g., %VAR%, !VAR!)
        "variable" => {
            if let Ok(var_name) = node.utf8_text(content.as_bytes()) {
                let clean = var_name.trim().to_string();
                if !clean.is_empty() {
                    data.variables.push(clean);
                }
            }
        }

        // Labels (e.g., :label_name) - treated as functions
        "label" => {
            if let Ok(label) = node.utf8_text(content.as_bytes()) {
                let clean = label.trim_start_matches(':').trim().to_string();
                if !clean.is_empty() {
                    data.functions.push(clean);
                }
            }
        }

        // Commands (e.g., call, goto, set) - treated as imports/dependencies
        "command" => {
            if let Ok(cmd) = node.utf8_text(content.as_bytes()) {
                let clean = cmd.trim().to_string();
                if !clean.is_empty() {
                    data.imports.push(clean);
                }
            }
        }

        // Regular comments (REM or ::)
        "comment" => {
            if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                let clean = comment.trim().to_string();
                if !clean.is_empty() {
                    data.comments.push(clean);
                }
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

/// Sorts and deduplicates all extracted data
fn clean_data(data: &mut AstData) {
    let fields = [
        &mut data.functions,
        &mut data.variables,
        &mut data.comments,
        &mut data.imports,
    ];
    for field in fields {
        field.sort();
        field.dedup();
    }
}
