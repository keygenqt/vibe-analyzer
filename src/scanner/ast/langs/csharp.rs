//! C# language AST parser.
//! This module provides functionality to parse C# source code into an abstract syntax tree (AST) representation.

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
use crate::scanner::ast::utils::extract_signature;

/// Parser for C# language files.
#[derive(Default)]
pub struct CSharpParser;

impl LanguageParser for CSharpParser {
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_c_sharp::LANGUAGE.into())?;

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
            // Only capture top-level comments that are not inside methods
            if !is_inside_method(node) {
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

        // Global statements with functions (top-level functions in C# 9+)
        "global_statement" => {
            if let Some(signature) = extract_signature(node, content) {
                let mut has_function = false;
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "local_function_statement" {
                        has_function = true;
                        break;
                    }
                }

                if has_function {
                    let comments = extract_csharp_doc_comments(node, content);
                    functions.push(AstDataFunction {
                        signature,
                        comments,
                    });
                }
            }
        }

        // Methods in classes
        "method_declaration" => {
            if let Some(signature) = extract_signature(node, content) {
                functions.push(AstDataFunction {
                    signature,
                    comments: extract_csharp_doc_comments(node, content),
                });
            }
        }

        // Record declarations (C# 9+)
        "record_declaration" => {
            if let Some(signature) = extract_signature(node, content) {
                structs.push(AstDataStruct {
                    signature,
                    comments: extract_csharp_doc_comments(node, content),
                });
            }
        }

        // Structs
        "struct_declaration" => {
            if let Some(signature) = extract_signature(node, content) {
                structs.push(AstDataStruct {
                    signature,
                    comments: extract_csharp_doc_comments(node, content),
                });
            }
        }

        // Classes
        "class_declaration" => {
            if let Some(signature) = extract_signature(node, content) {
                classes.push(AstDataClass {
                    signature,
                    comments: extract_csharp_doc_comments(node, content),
                });
            }
        }

        // Enums
        "enum_declaration" => {
            if let Some(signature) = extract_signature(node, content) {
                enums.push(AstDataEnum {
                    signature,
                    comments: extract_csharp_doc_comments(node, content),
                });
            }
        }

        // Interfaces
        "interface_declaration" => {
            if let Some(signature) = extract_signature(node, content) {
                interfaces.push(AstDataInterface {
                    signature,
                    comments: extract_csharp_doc_comments(node, content),
                });
            }
        }

        // Using directives (imports)
        "using_directive" => {
            if let Ok(text) = node.utf8_text(content.as_bytes()) {
                let cleaned = text
                    .replace("using ", "")
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

/// Extracts doc comments (///) before a node.
fn extract_csharp_doc_comments(node: &Node, content: &str) -> Vec<String> {
    let mut comments = Vec::new();

    // Walk backwards through sibling nodes to find preceding comments
    let mut prev = node.prev_sibling();
    while let Some(sibling) = prev {
        let kind = sibling.kind();

        // Skip attribute lists as they are not comments
        if kind == "attribute_list" {
            prev = sibling.prev_sibling();
            continue;
        }

        if kind != "comment" {
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

/// Checks if a node is inside a method body.
fn is_inside_method(node: &Node) -> bool {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "method_declaration" || p.kind() == "local_function_statement" {
            return true;
        }
        parent = p.parent();
    }
    false
}
