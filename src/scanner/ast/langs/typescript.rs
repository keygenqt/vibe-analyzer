//! TypeScript language AST parser.
//! This module provides functionality to parse TypeScript source code into an abstract syntax tree (AST) representation.

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

/// Parser for TypeScript language files.
#[derive(Default)]
pub struct TypeScriptParser;

impl LanguageParser for TypeScriptParser {
    /// Returns the human-readable language name.
    fn language_name(&self) -> &'static str {
        "TypeScript"
    }

    /// Parses TypeScript source code and extracts AST data.
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())?;

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
        "comment" => {
            // Capture only top-level comments that are not inside functions
            if node.prev_sibling().is_none() && !is_inside_function(node) {
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

        // Arrow functions and variables (lexical declarations)
        "lexical_declaration" => {
            if !is_inside_class(node) && !is_inside_function(node) {
                // For exported declarations, look at the export statement node for comments
                let search_node = if node
                    .parent()
                    .map_or(false, |p| p.kind() == "export_statement")
                {
                    node.parent().unwrap()
                } else {
                    *node
                };

                let comments = extract_ts_doc_comments(&search_node, content);

                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "variable_declarator" {
                        // Check if this is an arrow function assignment
                        if let Some(signature) = extract_ts_arrow_signature(&child, content) {
                            functions.push(AstDataFunction {
                                signature,
                                comments: comments.clone(),
                            });
                        } else if let Some(signature) =
                            extract_ts_variable_signature(node, &child, content)
                        {
                            variables.push(AstDataVariable {
                                signature,
                                comments: comments.clone(),
                            });
                        }
                    }
                }
            }
        }

        // Classes
        "class_declaration" => {
            // For exported declarations, look at the export statement node for comments
            let search_node = if node
                .parent()
                .map_or(false, |p| p.kind() == "export_statement")
            {
                node.parent().unwrap()
            } else {
                *node
            };

            let comments = extract_ts_doc_comments(&search_node, content);

            if let Some(signature) = extract_ts_signature_with_export(node, content) {
                classes.push(AstDataClass {
                    signature,
                    comments,
                });
            }
        }

        // Interfaces
        "interface_declaration" => {
            let search_node = if node
                .parent()
                .map_or(false, |p| p.kind() == "export_statement")
            {
                node.parent().unwrap()
            } else {
                *node
            };

            let comments = extract_ts_doc_comments(&search_node, content);

            if let Some(signature) = extract_ts_signature_with_export(node, content) {
                interfaces.push(AstDataInterface {
                    signature,
                    comments,
                });
            }
        }

        // Enums
        "enum_declaration" => {
            let search_node = if node
                .parent()
                .map_or(false, |p| p.kind() == "export_statement")
            {
                node.parent().unwrap()
            } else {
                *node
            };

            let comments = extract_ts_doc_comments(&search_node, content);

            if let Some(signature) = extract_ts_signature_with_export(node, content) {
                enums.push(AstDataEnum {
                    signature,
                    comments,
                });
            }
        }

        // Functions
        "function_declaration" => {
            if let Some(signature) = extract_ts_signature_with_export(node, content) {
                functions.push(AstDataFunction {
                    signature,
                    comments: extract_ts_doc_comments(node, content),
                });
            }
        }

        // Imports
        "import_statement" => {
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

/// Extracts arrow function signature from variable declarator.
fn extract_ts_arrow_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        if text.contains("=>") {
            if let Some(arrow_pos) = text.find("=>") {
                let before_arrow = &text[..arrow_pos].trim();
                let parts: Vec<&str> = before_arrow.split('=').collect();
                if parts.len() >= 2 {
                    let name = parts[0].trim();
                    let arrow_part = parts[1].trim();
                    return Some(format!("const {} = {}", name, arrow_part));
                }
                return Some(format!("const {}", before_arrow));
            }
        }
    }
    None
}

/// Extracts variable signature (const/let/var name).
fn extract_ts_variable_signature(parent: &Node, child: &Node, content: &str) -> Option<String> {
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

    let signature = format!("{} {}", prefix, name).trim().to_string();
    if !signature.is_empty() && signature.len() < 100 {
        Some(signature)
    } else {
        None
    }
}

/// Extracts signature with export prefix and without body.
fn extract_ts_signature_with_export(node: &Node, content: &str) -> Option<String> {
    // Check for export keyword before the node
    let mut has_export = false;
    let mut prev = node.prev_sibling();
    while let Some(sibling) = prev {
        if sibling.kind() == "export" {
            has_export = true;
            break;
        }
        prev = sibling.prev_sibling();
    }

    let prefix = if has_export { "export " } else { "" };

    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        // Trim everything after the opening brace
        if let Some(pos) = text.find('{') {
            let signature = text[..pos].trim().to_string();
            if !signature.is_empty() {
                return Some(format!("{}{}", prefix, signature));
            }
        }
        let first_line = text.lines().next().unwrap_or(text);
        let signature = first_line.trim().to_string();
        if !signature.is_empty() {
            return Some(format!("{}{}", prefix, signature));
        }
    }
    None
}

/// Extracts doc comments before a node.
fn extract_ts_doc_comments(node: &Node, content: &str) -> Vec<String> {
    let mut comments = Vec::new();

    // Walk backwards through sibling nodes to find preceding comments
    let mut prev = node.prev_sibling();
    while let Some(sibling) = prev {
        if sibling.kind() != "comment" {
            break;
        }

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

/// Checks if a node is inside a function.
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
