//! Go language AST parser.
//! This module provides functionality to parse Go source code into an abstract syntax tree (AST) representation.

use anyhow::Result;
use tree_sitter::Node;
use tree_sitter::Parser;

use crate::scanner::ast::data::ast_data::AstData;
use crate::scanner::ast::data::ast_data_enum::AstDataEnum;
use crate::scanner::ast::data::ast_data_function::AstDataFunction;
use crate::scanner::ast::data::ast_data_struct::AstDataStruct;
use crate::scanner::ast::data::ast_data_variable::AstDataVariable;
use crate::scanner::ast::traits::LanguageParser;

/// Parser for Go language files.
#[derive(Default)]
pub struct GoParser;

impl LanguageParser for GoParser {
    /// Returns the human-readable language name.
    fn language_name(&self) -> &'static str {
        "Go"
    }

    /// Parses Go source code and extracts AST data.
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_go::LANGUAGE.into())?;

        let tree = match parser.parse(content, None) {
            Some(tree) => tree,
            None => return Ok(AstData::default()),
        };

        let root_node = tree.root_node();
        let mut data = AstData::default();
        let mut functions = Vec::new();
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        let mut variables = Vec::new();

        // Traverse the AST and collect all declarations
        visit_node(
            &root_node,
            content,
            &mut data,
            &mut functions,
            &mut structs,
            &mut enums,
            &mut variables,
        );

        data.functions = functions;
        data.structs = structs;
        data.enums = enums;
        data.variables = variables;

        Ok(data.with_tags())
    }
}

/// Recursively traverses the AST and extracts relevant nodes.
fn visit_node(
    node: &Node,
    content: &str,
    data: &mut AstData,
    functions: &mut Vec<AstDataFunction>,
    structs: &mut Vec<AstDataStruct>,
    enums: &mut Vec<AstDataEnum>,
    variables: &mut Vec<AstDataVariable>,
) {
    match node.kind() {
        // Module comments (//)
        "comment" => {
            // Capture only top-level comments that are not inside functions
            if node.prev_sibling().is_none() && !is_inside_function(node) {
                if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                    let cleaned = comment.trim().to_string();
                    if cleaned.starts_with("//") {
                        let content = cleaned[2..].trim();
                        if !content.is_empty() {
                            data.header_comments.push(content.to_string());
                        }
                    }
                }
            }
        }

        // Top-level functions (without receiver)
        "function_declaration" => {
            // Skip methods (functions with receivers)
            if !is_method(node) {
                if let Some(signature) = extract_go_function_signature(node, content) {
                    functions.push(AstDataFunction {
                        signature,
                        comments: extract_go_doc_comments(node, content),
                    });
                }
            }
        }

        // Type declarations (structs and enums)
        "type_declaration" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "type_spec" {
                    let mut is_struct = false;
                    let mut type_name = String::new();
                    let mut builtin_type = String::new();
                    let mut type_cursor = child.walk();

                    for type_child in child.children(&mut type_cursor) {
                        if type_child.kind() == "struct_type" {
                            is_struct = true;
                        }
                        if type_child.kind() == "type_identifier" {
                            if type_name.is_empty() {
                                if let Ok(text) = type_child.utf8_text(content.as_bytes()) {
                                    type_name = text.to_string();
                                }
                            } else if let Ok(text) = type_child.utf8_text(content.as_bytes()) {
                                builtin_type = text.to_string();
                            }
                        }
                    }

                    if is_struct {
                        let signature = format!("type {} struct", type_name);
                        structs.push(AstDataStruct {
                            signature,
                            comments: extract_go_doc_comments(&child, content),
                        });
                    } else if !builtin_type.is_empty() && is_builtin_type(&builtin_type) {
                        // Type aliases to built-in types are treated as enums
                        enums.push(AstDataEnum {
                            signature: format!("type {} {}", type_name, builtin_type),
                            comments: extract_go_doc_comments(&child, content),
                        });
                    }
                }
            }
        }

        // Constants
        "const_declaration" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "const_spec" {
                    if let Some(signature) = extract_go_const_signature(&child, content) {
                        // Skip constants that have a type (they are enum values) and iota constants
                        let has_type = signature.split_whitespace().count() >= 3;
                        if !has_type && !signature.contains("iota") {
                            variables.push(AstDataVariable {
                                signature,
                                comments: extract_go_doc_comments(&child, content),
                            });
                        }
                    }
                }
            }
        }

        // Variables
        "var_declaration" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "var_spec" {
                    if let Some(signature) = extract_go_var_signature(&child, content) {
                        variables.push(AstDataVariable {
                            signature,
                            comments: extract_go_doc_comments(&child, content),
                        });
                    }
                }
            }
        }

        // Imports
        "import_declaration" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "import_spec_list" {
                    let mut spec_cursor = child.walk();
                    for spec in child.children(&mut spec_cursor) {
                        if spec.kind() == "import_spec" {
                            if let Ok(text) = spec.utf8_text(content.as_bytes()) {
                                let cleaned = text.replace("\"", "").trim().to_string();
                                if !cleaned.is_empty() && !data.imports.contains(&cleaned) {
                                    data.imports.push(cleaned);
                                }
                            }
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
        visit_node(&child, content, data, functions, structs, enums, variables);
    }
}

/// Checks if a type name is a built-in Go type.
fn is_builtin_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "int"
            | "int8"
            | "int16"
            | "int32"
            | "int64"
            | "uint"
            | "uint8"
            | "uint16"
            | "uint32"
            | "uint64"
            | "float32"
            | "float64"
            | "complex64"
            | "complex128"
            | "byte"
            | "rune"
            | "string"
            | "bool"
            | "error"
    )
}

/// Extracts function signature without body.
fn extract_go_function_signature(node: &Node, content: &str) -> Option<String> {
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

/// Extracts const signature (e.g., "const NAME" or "const NAME TYPE").
fn extract_go_const_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let parts: Vec<&str> = text.split_whitespace().collect();
        if parts.len() >= 2 {
            let name = parts[0];
            let type_part = if parts.len() > 2 { parts[1] } else { "" };
            if !type_part.is_empty() && type_part != "=" {
                return Some(format!("const {} {}", name, type_part));
            }
            return Some(format!("const {}", name));
        }
    }
    None
}

/// Extracts var signature (e.g., "var NAME" or "var NAME TYPE").
fn extract_go_var_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let parts: Vec<&str> = text.split_whitespace().collect();
        if parts.len() >= 2 {
            let name = parts[0];
            let type_part = if parts.len() > 2 { parts[1] } else { "" };
            if !type_part.is_empty() && type_part != "=" {
                return Some(format!("var {} {}", name, type_part));
            }
            return Some(format!("var {}", name));
        }
    }
    None
}

/// Checks if a function has a receiver (i.e., is a method).
fn is_method(node: &Node) -> bool {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "method_declaration" {
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
        if p.kind() == "function_declaration" || p.kind() == "method_declaration" {
            return true;
        }
        parent = p.parent();
    }
    false
}

/// Extracts doc comments before a node.
fn extract_go_doc_comments(node: &Node, content: &str) -> Vec<String> {
    // For spec nodes, look at the parent (declaration) for comments
    let search_node =
        if node.kind() == "type_spec" || node.kind() == "const_spec" || node.kind() == "var_spec" {
            if let Some(parent) = node.parent() {
                parent
            } else {
                *node
            }
        } else {
            *node
        };

    let start_byte = search_node.range().start_byte;
    let parent_start = search_node
        .parent()
        .map(|p| p.range().start_byte)
        .unwrap_or(0);
    let prefix_text = &content[parent_start..start_byte];

    // Parse lines in reverse to find consecutive comment lines
    let lines: Vec<&str> = prefix_text.lines().collect();
    let mut in_comment_block = false;
    let mut comment_lines: Vec<String> = Vec::new();

    for line in lines.iter().rev() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") {
            in_comment_block = true;
            comment_lines.push(trimmed[2..].trim().to_string());
        } else if in_comment_block && trimmed.is_empty() {
            // Empty line breaks the comment block
            break;
        } else if in_comment_block && !trimmed.is_empty() && !trimmed.starts_with("//") {
            break;
        }
    }

    // Reverse to restore original order
    comment_lines.reverse();
    comment_lines
}
