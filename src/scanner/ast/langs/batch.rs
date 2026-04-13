//! Batch script language AST parser.
//! This module provides functionality to parse Windows Batch script source code into an abstract syntax tree (AST) representation.

use anyhow::Result;
use tree_sitter::Node;
use tree_sitter::Parser;

use crate::scanner::ast::data::ast_data::AstData;
use crate::scanner::ast::data::ast_data_function::AstDataFunction;
use crate::scanner::ast::data::ast_data_variable::AstDataVariable;
use crate::scanner::ast::traits::LanguageParser;

/// Parser for Windows Batch language files.
#[derive(Default)]
pub struct BatchParser;

impl LanguageParser for BatchParser {
    /// Returns the human-readable language name.
    fn language_name(&self) -> &'static str {
        "Batch"
    }

    /// Parses Batch source code and extracts AST data.
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_batch::LANGUAGE.into())?;

        let tree = match parser.parse(content, None) {
            Some(tree) => tree,
            None => return Ok(AstData::default()),
        };

        let root_node = tree.root_node();
        let mut data = AstData::default();
        let mut functions = Vec::new();
        let mut variables = Vec::new();

        // Traverse the AST and collect all declarations
        visit_node(
            &root_node,
            content,
            &mut data,
            &mut functions,
            &mut variables,
        );

        data.functions = functions;
        data.variables = variables;

        Ok(data)
    }
}

/// Recursively traverses the AST and extracts relevant nodes.
fn visit_node(
    node: &Node,
    content: &str,
    data: &mut AstData,
    functions: &mut Vec<AstDataFunction>,
    variables: &mut Vec<AstDataVariable>,
) {
    match node.kind() {
        // Module comments (::)
        "comment" => {
            // Skip comments that belong to labels (they are extracted by extract_batch_doc_comments)
            if !is_inside_label(node) {
                // Check if this comment is directly followed by a label
                let next = node.next_sibling();
                if let Some(n) = next {
                    if n.kind() == "label" {
                        return;
                    }
                }

                if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                    let cleaned = comment.trim().to_string();
                    if cleaned.starts_with("::") {
                        let content = cleaned.trim_start_matches(':').trim().to_string();
                        if !content.is_empty() {
                            data.header_comments.push(content);
                        }
                    }
                }
            }
        }

        // Variable assignments
        "variable_assignment" => {
            if let Ok(text) = node.utf8_text(content.as_bytes()) {
                let signature = text.trim().to_string();
                let clean_sig = signature.replace("set ", "").trim().to_string();

                // Skip local variables (command line arguments and loop variables)
                if clean_sig.contains("%1") || clean_sig.contains("%2") || clean_sig.contains("%%i")
                {
                    return;
                }

                // Skip variables defined inside functions (labels)
                if is_inside_label(node) {
                    return;
                }

                if !clean_sig.is_empty() {
                    variables.push(AstDataVariable {
                        signature: clean_sig,
                        comments: vec![],
                    });
                }
            }
        }

        // Labels (functions)
        "label" => {
            if let Ok(label) = node.utf8_text(content.as_bytes()) {
                let clean = label.trim_start_matches(':').trim().to_string();
                if !clean.is_empty() {
                    functions.push(AstDataFunction {
                        signature: format!(":{}", clean),
                        comments: extract_batch_doc_comments(node, content),
                    });
                }
            }
        }

        // Import commands
        "call_stmt" => {
            if let Ok(text) = node.utf8_text(content.as_bytes()) {
                let trimmed = text.trim();
                if trimmed.starts_with("call ") {
                    let path = trimmed.trim_start_matches("call ").trim().to_string();
                    if !path.is_empty() && !data.imports.contains(&path) {
                        data.imports.push(path);
                    }
                }
            }
        }

        _ => {}
    }

    // Recursively process all child nodes
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_node(&child, content, data, functions, variables);
    }
}

/// Extracts doc comments (::) before a label.
fn extract_batch_doc_comments(node: &Node, content: &str) -> Vec<String> {
    let mut comments = Vec::new();

    // Walk backwards through sibling nodes to find preceding comments
    let mut prev = node.prev_sibling();
    while let Some(sibling) = prev {
        if sibling.kind() != "comment" {
            break;
        }

        if let Ok(text) = sibling.utf8_text(content.as_bytes()) {
            let cleaned = text.trim().to_string();
            if cleaned.starts_with("::") {
                let content = cleaned.trim_start_matches(':').trim().to_string();
                if !content.is_empty() {
                    comments.push(content);
                }
            }
        }

        prev = sibling.prev_sibling();
    }

    // Reverse to maintain original order (closest comment first)
    comments.reverse();
    comments
}

/// Checks if a node is inside a label body.
fn is_inside_label(node: &Node) -> bool {
    // Check preceding siblings for a label
    let mut prev = node.prev_sibling();
    while let Some(sibling) = prev {
        if sibling.kind() == "label" {
            return true;
        }
        prev = sibling.prev_sibling();
    }

    // Check ancestors for a label
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "label" {
            return true;
        }
        parent = p.parent();
    }
    false
}
