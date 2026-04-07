//! Bash/Shell language AST parser.
//! This module provides functionality to parse Bash/Shell source code into an abstract syntax tree (AST) representation.

use anyhow::Result;
use tree_sitter::Node;
use tree_sitter::Parser;

use crate::scanner::ast::data::ast_data::AstData;
use crate::scanner::ast::data::ast_data_function::AstDataFunction;
use crate::scanner::ast::data::ast_data_variable::AstDataVariable;
use crate::scanner::ast::traits::LanguageParser;

/// Parser for Bash/Shell language files.
#[derive(Default)]
pub struct BashParser;

impl LanguageParser for BashParser {
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_bash::LANGUAGE.into())?;

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
        // Module comments (## or ###)
        "comment" => {
            // Only capture top-level comments that are not inside functions
            if !is_inside_function(node) {
                if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                    let cleaned = comment.trim().to_string();
                    if cleaned.starts_with("##") {
                        let content = cleaned.trim_start_matches('#').trim().to_string();
                        if !content.is_empty() {
                            data.header_comments.push(content);
                        }
                    }
                }
            }
        }

        // Variables
        "variable_assignment" => {
            // Only capture global variables, not local ones inside functions
            if !is_inside_function(node) {
                if let Some(signature) = extract_bash_variable_signature(node, content) {
                    variables.push(AstDataVariable {
                        signature,
                        comments: extract_bash_doc_comments(node, content, true),
                    });
                }
            }
        }

        // Functions
        "function_definition" => {
            if let Some(signature) = extract_bash_function_signature(node, content) {
                functions.push(AstDataFunction {
                    signature,
                    comments: extract_bash_doc_comments(node, content, false),
                });
            }
        }

        // Source/import commands
        "source_command" => {
            println!("DEBUG source_command found!");
            if let Some(path) = extract_source_path(node, content) {
                println!("DEBUG source_command path: '{}'", path);
                data.imports.push(path);
            }
        }

        "command" => {
            if let Ok(text) = node.utf8_text(content.as_bytes()) {
                let trimmed = text.trim();
                // Handle 'source ./utils.sh' syntax
                if trimmed.starts_with("source ") {
                    let path = trimmed.trim_start_matches("source ").trim().to_string();
                    if !path.is_empty() {
                        data.imports.push(path);
                    }
                }
                // Handle '. ./config.sh' syntax (POSIX source)
                else if trimmed.starts_with(". ") {
                    let path = trimmed.trim_start_matches(". ").trim().to_string();
                    if !path.is_empty() {
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

/// Extracts variable signature from a variable assignment node.
fn extract_bash_variable_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let signature = text.trim().to_string();
        if !signature.is_empty() {
            return Some(signature);
        }
    }
    None
}

/// Extracts function signature (name with parentheses) from a function definition node.
fn extract_bash_function_signature(node: &Node, content: &str) -> Option<String> {
    let mut name = String::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "word" {
            if let Ok(text) = child.utf8_text(content.as_bytes()) {
                let cleaned = text.trim().to_string();
                // Skip the 'function' keyword itself
                if !cleaned.is_empty() && cleaned != "function" {
                    name = cleaned;
                    break;
                }
            }
        }
    }
    if !name.is_empty() {
        Some(format!("{}()", name))
    } else {
        None
    }
}

/// Extracts source file path from source command (source ./file.sh or . ./file.sh).
fn extract_source_path(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let trimmed = text.trim();
        if trimmed.starts_with("source ") {
            let path = trimmed.trim_start_matches("source ").trim().to_string();
            if !path.is_empty() {
                return Some(path);
            }
        }
    }
    None
}

/// Extracts doc comments (# or ##) for Bash nodes.
fn extract_bash_doc_comments(node: &Node, content: &str, is_variable: bool) -> Vec<String> {
    // Variables don't currently support doc comments
    if is_variable {
        return vec![];
    }

    let mut comments = Vec::new();
    let mut prev = node.prev_sibling();
    while let Some(sibling) = prev {
        if sibling.kind() != "comment" {
            break;
        }
        if let Ok(text) = sibling.utf8_text(content.as_bytes()) {
            let cleaned = text.trim().to_string();
            // Match single-line comments starting with '# ' or '#\t'
            if cleaned.starts_with("# ") || cleaned.starts_with("#\t") {
                let content = cleaned[1..].trim();
                if !content.is_empty() {
                    comments.push(content.to_string());
                }
            }
        }
        prev = sibling.prev_sibling();
    }
    // Reverse to maintain original order (closest comment first)
    comments.reverse();
    comments
}

/// Checks if a node is inside a function or method body.
fn is_inside_function(node: &Node) -> bool {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "function_definition" {
            return true;
        }
        parent = p.parent();
    }
    false
}
