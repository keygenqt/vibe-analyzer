//! Rust language AST parser

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{
    data::{
        ast_data::AstData, ast_data_enum::AstDataEnum, ast_data_function::AstDataFunction,
        ast_data_interface::AstDataInterface, ast_data_struct::AstDataStruct,
        ast_data_variable::AstDataVariable,
    },
    traits::LanguageParser,
    utils::{extract_import_path, extract_signature},
};

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
        let mut functions = Vec::new();
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        let mut interfaces = Vec::new();
        let mut variables = Vec::new();

        visit_node(
            &root_node,
            content,
            &mut data,
            &mut functions,
            &mut structs,
            &mut enums,
            &mut interfaces,
            &mut variables,
        );

        data.functions = functions;
        data.structs = structs;
        data.enums = enums;
        data.interfaces = interfaces;
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
    structs: &mut Vec<AstDataStruct>,
    enums: &mut Vec<AstDataEnum>,
    interfaces: &mut Vec<AstDataInterface>,
    variables: &mut Vec<AstDataVariable>,
) {
    match node.kind() {
        // Module comments (//!)
        "line_comment" | "block_comment" => {
            if !is_inside_function(node) {
                if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                    let cleaned = comment.trim().to_string();
                    if cleaned.starts_with("//!") {
                        let content = cleaned[3..].trim();
                        if !content.is_empty() {
                            data.comments.push(content.to_string());
                        }
                    }
                }
            }
        }

        // Global variables (const, static)
        "const_item" | "static_item" => {
            if let Some(signature) = extract_signature(node, content) {
                variables.push(AstDataVariable {
                    signature,
                    comments: extract_rust_doc_comments(node, content),
                });
            }
        }

        // Functions and methods
        "function_item" | "method_definition" => {
            if let Some(signature) = extract_signature(node, content) {
                functions.push(AstDataFunction {
                    signature,
                    comments: extract_rust_doc_comments(node, content),
                });
            }
        }

        // Structs
        "struct_item" => {
            if let Some(signature) = extract_signature(node, content) {
                structs.push(AstDataStruct {
                    signature,
                    comments: extract_rust_doc_comments(node, content),
                });
            }
        }

        // Enums
        "enum_item" => {
            if let Some(signature) = extract_signature(node, content) {
                enums.push(AstDataEnum {
                    signature,
                    comments: extract_rust_doc_comments(node, content),
                });
            }
        }

        // Traits
        "trait_item" => {
            if let Some(signature) = extract_signature(node, content) {
                interfaces.push(AstDataInterface {
                    signature,
                    comments: extract_rust_doc_comments(node, content),
                });
            }
        }

        // Imports
        "use_declaration" => {
            if let Some(path) = extract_import_path(node, content, "use ") {
                data.imports.push(path);
            }
        }

        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_node(
            &child, content, data, functions, structs, enums, interfaces, variables,
        );
    }
}

/// Extracts doc comments (///, /** */, /*!) before a node, skipping attributes
pub fn extract_rust_doc_comments(node: &Node, content: &str) -> Vec<String> {
    let mut comments = Vec::new();
    let mut prev = node.prev_sibling();

    while let Some(sibling) = prev {
        let kind = sibling.kind();

        if kind == "attribute_item" || kind == "attribute" || kind.contains("annotation") {
            prev = sibling.prev_sibling();
            continue;
        }

        if !kind.contains("comment") {
            break;
        }

        if let Ok(text) = sibling.utf8_text(content.as_bytes()) {
            let cleaned = text.trim().to_string();
            if cleaned.starts_with("///")
                || cleaned.starts_with("/**")
                || cleaned.starts_with("/*!")
            {
                let comment_text = if cleaned.starts_with("///") {
                    cleaned[3..].trim()
                } else {
                    let without_start = &cleaned[3..];
                    if without_start.ends_with("*/") {
                        without_start[..without_start.len() - 2].trim()
                    } else {
                        without_start.trim()
                    }
                };
                if !comment_text.is_empty() {
                    comments.push(comment_text.to_string());
                }
            }
        }

        prev = sibling.prev_sibling();
    }

    comments.reverse();
    comments
}

/// Checks if a node is inside a function body
fn is_inside_function(node: &Node) -> bool {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "function_item" || p.kind() == "method_definition" || p.kind() == "impl_item"
        {
            return true;
        }
        parent = p.parent();
    }
    false
}
