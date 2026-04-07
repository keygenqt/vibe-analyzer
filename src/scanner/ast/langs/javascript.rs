//! JavaScript language AST parser.
//! This module provides functionality to parse JavaScript source code into an abstract syntax tree (AST) representation.

use anyhow::Result;
use tree_sitter::Node;
use tree_sitter::Parser;

use crate::scanner::ast::data::ast_data::AstData;
use crate::scanner::ast::data::ast_data_class::AstDataClass;
use crate::scanner::ast::data::ast_data_function::AstDataFunction;
use crate::scanner::ast::data::ast_data_variable::AstDataVariable;
use crate::scanner::ast::traits::LanguageParser;
use crate::scanner::ast::utils::extract_signature;

/// Parser for JavaScript language files.
#[derive(Default)]
pub struct JavaScriptParser;

impl LanguageParser for JavaScriptParser {
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_javascript::LANGUAGE.into())?;

        let tree = match parser.parse(content, None) {
            Some(tree) => tree,
            None => return Ok(AstData::default()),
        };

        let root_node = tree.root_node();
        let mut data = AstData::default();
        let mut functions = Vec::new();
        let mut classes = Vec::new();
        let mut variables = Vec::new();

        // Traverse the AST and collect all declarations
        visit_node(
            &root_node,
            content,
            &mut data,
            &mut functions,
            &mut classes,
            &mut variables,
        );

        data.functions = functions;
        data.classes = classes;
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
    classes: &mut Vec<AstDataClass>,
    variables: &mut Vec<AstDataVariable>,
) {
    match node.kind() {
        // Module comments (/** */)
        "comment" => {
            // Capture only top-level comments with no preceding non-comment siblings
            if node.prev_sibling().is_none() || is_module_comment(node, content) {
                if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                    let cleaned = comment.trim().to_string();
                    if cleaned.starts_with("/**") {
                        // Extract the content between /** and */
                        let content = cleaned[3..cleaned.len() - 2].trim();
                        let lines: Vec<String> = content
                            .lines()
                            .map(|line| line.trim_start_matches('*').trim_start().to_string())
                            .collect();
                        let comment_text = lines.join(" ").trim().to_string();
                        if !comment_text.is_empty() {
                            data.header_comments.push(comment_text);
                        }
                    }
                }
            }
        }

        // Functions
        "function_declaration" => {
            // Skip functions defined inside other functions
            if !is_inside_function(node) {
                if let Some(signature) = extract_signature(node, content) {
                    functions.push(AstDataFunction {
                        signature,
                        comments: extract_js_doc_comments(node, content),
                    });
                }
            }
        }

        // Variables and arrow functions
        "lexical_declaration" => {
            if !is_inside_function(node) {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "variable_declarator" {
                        // Check if this is an arrow function assignment
                        if let Some(signature) = extract_js_arrow_signature(&child, content) {
                            functions.push(AstDataFunction {
                                signature,
                                comments: extract_js_doc_comments(node, content),
                            });
                        } else {
                            let signature = extract_js_variable_signature(node, &child, content);
                            variables.push(AstDataVariable {
                                signature,
                                comments: extract_js_doc_comments(node, content),
                            });
                        }
                    }
                }
            }
        }

        // Classes
        "class_declaration" => {
            if let Some(signature) = extract_signature(node, content) {
                classes.push(AstDataClass {
                    signature,
                    comments: extract_js_doc_comments(node, content),
                });
            }
        }

        // Imports
        "import_statement" => {
            if let Ok(text) = node.utf8_text(content.as_bytes()) {
                // Extract import path from double quotes
                if let Some(start) = text.find('"') {
                    let rest = &text[start + 1..];
                    if let Some(end) = rest.find('"') {
                        let path = rest[..end].to_string();
                        if !path.is_empty() && !data.imports.contains(&path) {
                            data.imports.push(path);
                        }
                    }
                // Extract import path from single quotes
                } else if let Some(start) = text.find('\'') {
                    let rest = &text[start + 1..];
                    if let Some(end) = rest.find('\'') {
                        let path = rest[..end].to_string();
                        if !path.is_empty() && !data.imports.contains(&path) {
                            data.imports.push(path);
                        }
                    }
                }
            }
        }

        _ => {}
    }

    // Recursively process all child nodes
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_node(&child, content, data, functions, classes, variables);
    }
}

/// Extracts arrow function signature from variable declarator.
fn extract_js_arrow_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        if text.contains("=>") {
            // Extract the part before the arrow function body
            if let Some(pos) = text.find("=>") {
                let before_arrow = &text[..pos + 2];
                let parts: Vec<&str> = before_arrow.split('=').collect();
                if parts.len() >= 2 {
                    let name = parts[0].trim();
                    let arrow_part = parts[1].trim();
                    return Some(format!("const {} = {}", name, arrow_part));
                }
            }
            // Fallback: split at => and take first part
            let signature = text.split("=>").next().unwrap_or(text).trim().to_string();
            if !signature.is_empty() && signature.len() < 100 {
                return Some(format!("{} =>", signature));
            }
        }
    }
    None
}

/// Extracts variable signature with const/let/export prefix.
fn extract_js_variable_signature(parent: &Node, child: &Node, content: &str) -> String {
    let mut prefix = String::new();

    // Check if this variable is exported
    if let Some(grandparent) = parent.parent() {
        if grandparent.kind() == "export_statement" {
            prefix = "export ".to_string();
        }
    }

    // Get const/let/var keyword
    if let Ok(text) = parent.utf8_text(content.as_bytes()) {
        let parts: Vec<&str> = text.split_whitespace().collect();
        if !parts.is_empty() && (parts[0] == "const" || parts[0] == "let" || parts[0] == "var") {
            prefix.push_str(parts[0]);
        }
    }

    // Get variable name (without assignment)
    let name = if let Ok(text) = child.utf8_text(content.as_bytes()) {
        text.split('=').next().unwrap_or(text).trim().to_string()
    } else {
        String::new()
    };

    // Return without equals sign
    format!("{} {}", prefix, name).trim().to_string()
}

/// Extracts doc comments (/** */) before a node.
fn extract_js_doc_comments(node: &Node, content: &str) -> Vec<String> {
    let mut comments = Vec::new();

    // Walk backwards through sibling nodes to find preceding comments
    let mut prev = node.prev_sibling();
    while let Some(sibling) = prev {
        let kind = sibling.kind();

        if kind != "comment" {
            break;
        }

        if let Ok(text) = sibling.utf8_text(content.as_bytes()) {
            let cleaned = text.trim().to_string();
            if cleaned.starts_with("/**") {
                // Extract the content between /** and */
                let content = cleaned[3..cleaned.len() - 2].trim();
                let lines: Vec<String> = content
                    .lines()
                    .map(|line| line.trim_start_matches('*').trim_start().to_string())
                    .filter(|line| !line.is_empty())
                    .collect();
                for line in lines {
                    if !line.is_empty() {
                        // Remove leading * if present
                        let clean_line = line.trim_start_matches('*').trim_start().to_string();
                        comments.push(clean_line);
                    }
                }
            }
        }

        prev = sibling.prev_sibling();
    }

    comments
}

/// Checks if a node is inside a function body.
fn is_inside_function(node: &Node) -> bool {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "function_declaration" || p.kind() == "arrow_function" {
            return true;
        }
        parent = p.parent();
    }
    false
}

/// Checks if a comment is a module-level comment (only comments before it).
fn is_module_comment(node: &Node, _content: &str) -> bool {
    let mut prev = node.prev_sibling();
    while let Some(sibling) = prev {
        if sibling.kind() != "comment" {
            return false;
        }
        prev = sibling.prev_sibling();
    }
    true
}
