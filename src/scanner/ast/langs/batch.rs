//! Batch script language AST parser

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{
    data::{
        ast_data::AstData, ast_data_function::AstDataFunction, ast_data_variable::AstDataVariable,
    },
    traits::LanguageParser,
};

/// Parser for Windows Batch language files
#[derive(Default)]
pub struct BatchParser;

impl LanguageParser for BatchParser {
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

/// Recursively traverses the AST and extracts relevant nodes
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
            if !is_inside_label(node) {
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
                            data.comments.push(content);
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

                // Skip local variables
                if clean_sig.contains("%1") || clean_sig.contains("%2") || clean_sig.contains("%%i")
                {
                    return;
                }

                // Skip variables inside functions
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

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_node(&child, content, data, functions, variables);
    }
}

/// Extracts doc comments (::) before a label
fn extract_batch_doc_comments(node: &Node, content: &str) -> Vec<String> {
    let mut comments = Vec::new();

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

    comments.reverse();
    comments
}

/// Checks if a node is inside a label body
fn is_inside_label(node: &Node) -> bool {
    let mut prev = node.prev_sibling();
    while let Some(sibling) = prev {
        if sibling.kind() == "label" {
            return true;
        }
        prev = sibling.prev_sibling();
    }

    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "label" {
            return true;
        }
        parent = p.parent();
    }
    false
}
