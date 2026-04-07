//! Dart language AST parser.
//! This module provides functionality to parse Dart source code into an abstract syntax tree (AST) representation.

use anyhow::Result;
use tree_sitter::Node;
use tree_sitter::Parser;

use crate::scanner::ast::data::ast_data::AstData;
use crate::scanner::ast::data::ast_data_class::AstDataClass;
use crate::scanner::ast::data::ast_data_enum::AstDataEnum;
use crate::scanner::ast::data::ast_data_function::AstDataFunction;
use crate::scanner::ast::data::ast_data_interface::AstDataInterface;
use crate::scanner::ast::data::ast_data_variable::AstDataVariable;
use crate::scanner::ast::traits::LanguageParser;
use crate::scanner::ast::utils::extract_signature;

/// Parser for Dart language files.
#[derive(Default)]
pub struct DartParser;

impl LanguageParser for DartParser {
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_dart::LANGUAGE.into())?;

        let tree = match parser.parse(content, None) {
            Some(tree) => tree,
            None => return Ok(AstData::default()),
        };

        let root_node = tree.root_node();
        let mut data = AstData::default();
        let mut functions = Vec::new();
        let mut classes = Vec::new();
        let mut enums = Vec::new();
        let mut interfaces = Vec::new();
        let mut variables = Vec::new();

        // Traverse the AST and collect all declarations
        visit_node(
            &root_node,
            content,
            &mut data,
            &mut functions,
            &mut classes,
            &mut enums,
            &mut interfaces,
            &mut variables,
        );

        data.functions = functions;
        data.classes = classes;
        data.enums = enums;
        data.interfaces = interfaces;
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
    enums: &mut Vec<AstDataEnum>,
    interfaces: &mut Vec<AstDataInterface>,
    variables: &mut Vec<AstDataVariable>,
) {
    match node.kind() {
        // Module comments (///)
        "comment" => {
            // Capture only top-level comments that are module documentation
            if node.prev_sibling().is_none() || is_module_comment(node, content) {
                if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                    let cleaned = comment.trim().to_string();
                    if cleaned.starts_with("///") {
                        let content = cleaned[3..].trim().to_string();
                        if !content.is_empty() {
                            data.header_comments.push(content);
                        }
                    }
                }
            }
        }

        // Top-level functions
        "function_signature" => {
            if !is_inside_class(node) {
                if let Some(signature) = extract_signature(node, content) {
                    functions.push(AstDataFunction {
                        signature,
                        comments: extract_dart_doc_comments(node, content),
                    });
                }
            }
        }

        // Classes
        "class_declaration" => {
            if let Some(signature) = extract_signature(node, content) {
                classes.push(AstDataClass {
                    signature,
                    comments: extract_dart_doc_comments(node, content),
                });
            }
        }

        // Mixins as interfaces
        "mixin_declaration" => {
            if let Some(signature) = extract_signature(node, content) {
                interfaces.push(AstDataInterface {
                    signature,
                    comments: extract_dart_doc_comments(node, content),
                });
            }
        }

        // Enums
        "enum_declaration" => {
            if let Some(signature) = extract_signature(node, content) {
                enums.push(AstDataEnum {
                    signature,
                    comments: extract_dart_doc_comments(node, content),
                });
            }
        }

        // Imports
        "import_specification" => {
            if let Ok(text) = node.utf8_text(content.as_bytes()) {
                // Extract import path from single quotes
                if let Some(start) = text.find('\'') {
                    let rest = &text[start + 1..];
                    if let Some(end) = rest.find('\'') {
                        let path = rest[..end].to_string();
                        if !path.is_empty() && !data.imports.contains(&path) {
                            data.imports.push(path);
                        }
                    }
                // Extract import path from double quotes
                } else if let Some(start) = text.find('"') {
                    let rest = &text[start + 1..];
                    if let Some(end) = rest.find('"') {
                        let path = rest[..end].to_string();
                        if !path.is_empty() && !data.imports.contains(&path) {
                            data.imports.push(path);
                        }
                    }
                }
            }
        }

        // Global variables (initialized variables and static final declarations)
        "initialized_identifier_list" | "static_final_declaration_list" => {
            if !is_inside_class(node) && !is_inside_function(node) {
                let mut parts = Vec::new();
                let mut first_token = *node;

                // Collect prefixes (late, const, final, type)
                let mut prev = node.prev_sibling();
                while let Some(sibling) = prev {
                    let kind = sibling.kind();
                    if kind == ";" || kind == "comment" || kind == "///" {
                        break;
                    }

                    if let Ok(text) = sibling.utf8_text(content.as_bytes()) {
                        let t = text.trim();
                        if !t.is_empty() {
                            parts.insert(0, t.to_string());
                            first_token = sibling;
                        }
                    }
                    prev = sibling.prev_sibling();
                }

                // Add variable name (without value assignment)
                if let Ok(text) = node.utf8_text(content.as_bytes()) {
                    let name_only = text.split('=').next().unwrap_or(text).trim();
                    parts.push(name_only.to_string());
                }

                let signature = parts.join(" ");

                if !signature.is_empty() && !variables.iter().any(|v| v.signature == signature) {
                    let comments = extract_dart_doc_comments(&first_token, content);
                    variables.push(AstDataVariable {
                        signature,
                        comments,
                    });
                }
            }
        }

        _ => {}
    }

    // Recursively process all child nodes
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_node(
            &child, content, data, functions, classes, enums, interfaces, variables,
        );
    }
}

/// Extracts doc comments (///) before a node.
fn extract_dart_doc_comments(node: &Node, content: &str) -> Vec<String> {
    let mut comments = Vec::new();

    // Walk backwards through sibling nodes to find preceding comments
    let mut prev = node.prev_sibling();
    while let Some(sibling) = prev {
        if sibling.kind() != "comment" {
            break;
        }

        if let Ok(text) = sibling.utf8_text(content.as_bytes()) {
            let cleaned = text.trim().to_string();
            if cleaned.starts_with("///") {
                let content = cleaned[3..].trim().to_string();
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

/// Checks if a node is inside a class.
fn is_inside_class(node: &Node) -> bool {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "class_declaration" {
            return true;
        }
        parent = p.parent();
    }
    false
}

/// Checks if a node is inside a function.
fn is_inside_function(node: &Node) -> bool {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "function_signature" {
            return true;
        }
        parent = p.parent();
    }
    false
}

/// Checks if a comment is a module-level comment (no preceding non-comment siblings).
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
