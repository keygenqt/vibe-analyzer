//! Java language AST parser.
//! This module provides functionality to parse Java source code into an abstract syntax tree (AST) representation.

use anyhow::Result;
use tree_sitter::Node;
use tree_sitter::Parser;

use crate::scanner::ast::data::ast_data::AstData;
use crate::scanner::ast::data::ast_data_class::AstDataClass;
use crate::scanner::ast::data::ast_data_enum::AstDataEnum;
use crate::scanner::ast::data::ast_data_function::AstDataFunction;
use crate::scanner::ast::data::ast_data_interface::AstDataInterface;
use crate::scanner::ast::data::ast_data_struct::AstDataStruct;
use crate::scanner::ast::data::ast_data_variable::AstDataVariable;
use crate::scanner::ast::traits::LanguageParser;

/// Parser for Java language files.
#[derive(Default)]
pub struct JavaParser;

impl LanguageParser for JavaParser {
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_java::LANGUAGE.into())?;

        let tree = match parser.parse(content, None) {
            Some(tree) => tree,
            None => return Ok(AstData::default()),
        };

        let root_node = tree.root_node();
        let mut data = AstData::default();
        let mut functions = Vec::new();
        let mut structs = Vec::new();
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
            &mut structs,
            &mut classes,
            &mut enums,
            &mut interfaces,
            &mut variables,
        );

        data.functions = functions;
        data.structs = structs;
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
    structs: &mut Vec<AstDataStruct>,
    classes: &mut Vec<AstDataClass>,
    enums: &mut Vec<AstDataEnum>,
    interfaces: &mut Vec<AstDataInterface>,
    variables: &mut Vec<AstDataVariable>,
) {
    match node.kind() {
        // Module comments (/** */)
        "block_comment" => {
            // Capture only top-level comments with no preceding siblings
            if node.prev_sibling().is_none() {
                if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                    let cleaned = comment.trim().to_string();
                    if cleaned.starts_with("/**") {
                        // Extract the content between /** and */
                        let body = cleaned[3..cleaned.len() - 2].trim();
                        let lines: Vec<String> = body
                            .lines()
                            .map(|line| line.trim_start_matches('*').trim_start().to_string())
                            .filter(|line| !line.is_empty())
                            .collect();
                        let comment_text = lines.join(" ").trim().to_string();
                        if !comment_text.is_empty() {
                            data.header_comments.push(comment_text);
                        }
                    }
                }
            }
        }

        // Methods (top-level methods in Java records and classes are handled elsewhere)
        "method_declaration" => {
            // Capture only top-level methods not inside interfaces or classes (rare in Java)
            if !is_inside_interface(node) && !is_inside_class(node) {
                if let Some(signature) = extract_java_method_signature(node, content) {
                    functions.push(AstDataFunction {
                        signature,
                        comments: extract_java_doc_comments(node, content),
                    });
                }
            }
        }

        // Classes
        "class_declaration" => {
            if let Some(signature) = extract_java_class_signature(node, content) {
                classes.push(AstDataClass {
                    signature,
                    comments: extract_java_doc_comments(node, content),
                });
            }
        }

        // Enums
        "enum_declaration" => {
            if let Some(signature) = extract_java_enum_signature(node, content) {
                enums.push(AstDataEnum {
                    signature,
                    comments: extract_java_doc_comments(node, content),
                });
            }
        }

        // Interfaces
        "interface_declaration" => {
            if let Some(signature) = extract_java_interface_signature(node, content) {
                interfaces.push(AstDataInterface {
                    signature,
                    comments: extract_java_doc_comments(node, content),
                });
            }
        }

        // Global variables (field declarations outside classes are rare in Java)
        "local_variable_declaration" => {
            if !is_inside_class(node) && !is_inside_method(node) {
                if let Some(signature) = extract_java_variable_signature(node, content) {
                    variables.push(AstDataVariable {
                        signature,
                        comments: extract_java_doc_comments(node, content),
                    });
                }
            }
        }

        // Imports
        "import_declaration" => {
            if let Ok(text) = node.utf8_text(content.as_bytes()) {
                let cleaned = text
                    .replace("import ", "")
                    .replace(";", "")
                    .trim()
                    .to_string();
                if !cleaned.is_empty() && !data.imports.contains(&cleaned) {
                    data.imports.push(cleaned);
                }
            }
        }

        _ => {}
    }

    // Recursively process all child nodes
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_node(
            &child, content, data, functions, structs, classes, enums, interfaces, variables,
        );
    }
}

/// Extracts method signature without body.
fn extract_java_method_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        // Trim everything after the opening brace
        if let Some(pos) = text.find('{') {
            let signature = text[..pos].trim().to_string();
            if !signature.is_empty() {
                return Some(signature);
            }
        }
        // Fallback to first line only
        let first_line = text.lines().next().unwrap_or(text);
        let signature = first_line.trim().to_string();
        if !signature.is_empty() {
            return Some(signature);
        }
    }
    None
}

/// Extracts class signature without body.
fn extract_java_class_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        if let Some(pos) = text.find('{') {
            let signature = text[..pos].trim().to_string();
            if !signature.is_empty() {
                return Some(signature);
            }
        }
        let first_line = text.lines().next().unwrap_or(text);
        let signature = first_line.trim().to_string();
        if !signature.is_empty() {
            return Some(signature);
        }
    }
    None
}

/// Extracts enum signature without body.
fn extract_java_enum_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        if let Some(pos) = text.find('{') {
            let signature = text[..pos].trim().to_string();
            if !signature.is_empty() {
                return Some(signature);
            }
        }
        let first_line = text.lines().next().unwrap_or(text);
        let signature = first_line.trim().to_string();
        if !signature.is_empty() {
            return Some(signature);
        }
    }
    None
}

/// Extracts interface signature without body.
fn extract_java_interface_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        if let Some(pos) = text.find('{') {
            let signature = text[..pos].trim().to_string();
            if !signature.is_empty() {
                return Some(signature);
            }
        }
        let first_line = text.lines().next().unwrap_or(text);
        let signature = first_line.trim().to_string();
        if !signature.is_empty() {
            return Some(signature);
        }
    }
    None
}

/// Extracts variable signature without value.
fn extract_java_variable_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let without_value = text.split('=').next().unwrap_or(text);
        let cleaned = without_value.trim().replace(';', "").trim().to_string();
        if !cleaned.is_empty() && cleaned.len() < 100 {
            return Some(cleaned);
        }
    }
    None
}

/// Extracts doc comments before a node.
fn extract_java_doc_comments(node: &Node, content: &str) -> Vec<String> {
    let mut comments = Vec::new();

    // Walk backwards through sibling nodes to find preceding block comments
    let mut prev = node.prev_sibling();
    while let Some(sibling) = prev {
        if sibling.kind() == "block_comment" {
            if let Ok(text) = sibling.utf8_text(content.as_bytes()) {
                let cleaned = text.trim().to_string();
                if cleaned.starts_with("/**") {
                    // Extract the content between /** and */
                    let body = cleaned[3..cleaned.len() - 2].trim();
                    let lines: Vec<String> = body
                        .lines()
                        .map(|line| line.trim().trim_start_matches('*').trim_start().to_string())
                        .filter(|line| !line.is_empty())
                        .collect();
                    for line in lines {
                        comments.push(line);
                    }
                }
            }
        } else if sibling.kind() != "line_comment" {
            break;
        }
        prev = sibling.prev_sibling();
    }

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

/// Checks if a node is inside a method.
fn is_inside_method(node: &Node) -> bool {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "method_declaration" {
            return true;
        }
        parent = p.parent();
    }
    false
}

/// Checks if a node is inside an interface.
fn is_inside_interface(node: &Node) -> bool {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "interface_declaration" {
            return true;
        }
        parent = p.parent();
    }
    false
}
