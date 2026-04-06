//! Swift language AST parser

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{
    data::{
        ast_data::AstData, ast_data_class::AstDataClass, ast_data_enum::AstDataEnum,
        ast_data_function::AstDataFunction, ast_data_interface::AstDataInterface,
        ast_data_struct::AstDataStruct, ast_data_variable::AstDataVariable,
    },
    traits::LanguageParser,
};

/// Parser for Swift language files
#[derive(Default)]
pub struct SwiftParser;

impl LanguageParser for SwiftParser {
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_swift::LANGUAGE.into())?;

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
        let mut protocols = Vec::new();
        let mut variables = Vec::new();

        visit_node(
            &root_node,
            content,
            &mut data,
            &mut functions,
            &mut structs,
            &mut classes,
            &mut enums,
            &mut protocols,
            &mut variables,
        );

        data.functions = functions;
        data.structs = structs;
        data.classes = classes;
        data.enums = enums;
        data.interfaces = protocols;
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
    classes: &mut Vec<AstDataClass>,
    enums: &mut Vec<AstDataEnum>,
    protocols: &mut Vec<AstDataInterface>,
    variables: &mut Vec<AstDataVariable>,
) {
    match node.kind() {
        // Module comments (///)
        "comment" => {
            let is_module = node.prev_sibling().is_none()
                || (node.prev_sibling().is_some()
                    && node.prev_sibling().unwrap().kind() == "comment");

            if is_module && !is_inside_function(node) {
                if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                    let cleaned = comment.trim().to_string();
                    if cleaned.starts_with("///") {
                        let content = cleaned[3..].trim();
                        if !content.is_empty() {
                            data.comments.push(content.to_string());
                        }
                    }
                }
            }
        }

        // Functions
        "function_declaration" => {
            if !is_inside_class(node) && !is_inside_struct(node) {
                if let Some(signature) = extract_swift_function_signature(node, content) {
                    functions.push(AstDataFunction {
                        signature,
                        comments: extract_swift_doc_comments(node, content),
                    });
                }
            }
        }

        // Structs
        "struct_declaration" => {
            if let Some(signature) = extract_swift_struct_signature(node, content) {
                structs.push(AstDataStruct {
                    signature,
                    comments: extract_swift_doc_comments(node, content),
                });
            }
        }

        // Classes, structs, enums (all come as class_declaration in Swift)
        "class_declaration" => {
            if let Ok(text) = node.utf8_text(content.as_bytes()) {
                let trimmed = text.trim();

                if trimmed.starts_with("struct ") {
                    if let Some(signature) = extract_swift_struct_signature(node, content) {
                        structs.push(AstDataStruct {
                            signature,
                            comments: extract_swift_doc_comments(node, content),
                        });
                    }
                } else if trimmed.starts_with("enum ") {
                    if let Some(signature) = extract_swift_enum_signature(node, content) {
                        enums.push(AstDataEnum {
                            signature,
                            comments: extract_swift_doc_comments(node, content),
                        });
                    }
                } else {
                    if let Some(signature) = extract_swift_class_signature(node, content) {
                        classes.push(AstDataClass {
                            signature,
                            comments: extract_swift_doc_comments(node, content),
                        });
                    }
                }
            }
        }

        // Enums
        "enum_declaration" => {
            if let Some(signature) = extract_swift_enum_signature(node, content) {
                enums.push(AstDataEnum {
                    signature,
                    comments: extract_swift_doc_comments(node, content),
                });
            }
        }

        // Protocols (interfaces)
        "protocol_declaration" => {
            if let Some(signature) = extract_swift_protocol_signature(node, content) {
                protocols.push(AstDataInterface {
                    signature,
                    comments: extract_swift_doc_comments(node, content),
                });
            }
        }

        // Global variables
        "property_declaration" => {
            if !is_inside_class(node) && !is_inside_struct(node) && !is_inside_function(node) {
                if let Some(signature) = extract_swift_variable_signature(node, content) {
                    variables.push(AstDataVariable {
                        signature,
                        comments: extract_swift_doc_comments(node, content),
                    });
                }
            }
        }

        // Imports
        "import_declaration" => {
            if let Ok(text) = node.utf8_text(content.as_bytes()) {
                let cleaned = text.replace("import ", "").trim().to_string();
                if !cleaned.is_empty() && !data.imports.contains(&cleaned) {
                    data.imports.push(cleaned);
                }
            }
        }

        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_node(
            &child, content, data, functions, structs, classes, enums, protocols, variables,
        );
    }
}

/// Extracts function signature without body
fn extract_swift_function_signature(node: &Node, content: &str) -> Option<String> {
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

/// Extracts struct signature without body
fn extract_swift_struct_signature(node: &Node, content: &str) -> Option<String> {
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

/// Extracts class signature without body
fn extract_swift_class_signature(node: &Node, content: &str) -> Option<String> {
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

/// Extracts enum signature without body
fn extract_swift_enum_signature(node: &Node, content: &str) -> Option<String> {
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

/// Extracts protocol signature without body
fn extract_swift_protocol_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let first_line = text.lines().next().unwrap_or(text);
        let signature = first_line.trim().to_string();
        if !signature.is_empty() {
            return Some(signature);
        }
    }
    None
}

/// Extracts variable signature without value
fn extract_swift_variable_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let signature = text.split('=').next().unwrap_or(text).trim().to_string();
        if !signature.is_empty() && !signature.contains('(') && signature.len() < 100 {
            return Some(signature);
        }
    }
    None
}

/// Extracts doc comments before a node
fn extract_swift_doc_comments(node: &Node, content: &str) -> Vec<String> {
    let start_byte = node.range().start_byte;
    let parent_start = node.parent().map(|p| p.range().start_byte).unwrap_or(0);
    let prefix_text = &content[parent_start..start_byte];

    if let Some(pos) = prefix_text.rfind("/**") {
        let between = &prefix_text[pos + 3..];
        if !between.contains("let ") && !between.contains("var ") {
            let comment_text = &prefix_text[pos..];
            if let Some(end) = comment_text.find("*/") {
                let full_comment = &comment_text[..end + 2];
                let body = full_comment
                    .trim_start_matches("/**")
                    .trim_end_matches("*/")
                    .trim();

                let lines: Vec<String> = body
                    .lines()
                    .map(|line| {
                        line.trim()
                            .trim_start_matches('*')
                            .trim_start()
                            .replace("* ", "")
                            .to_string()
                    })
                    .filter(|line| !line.is_empty())
                    .collect();

                if !lines.is_empty() {
                    return lines;
                }
            }
        }
    }

    vec![]
}

/// Checks if a node is inside a class
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

/// Checks if a node is inside a struct
fn is_inside_struct(node: &Node) -> bool {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "struct_declaration" {
            return true;
        }
        parent = p.parent();
    }
    false
}

/// Checks if a node is inside a function
fn is_inside_function(node: &Node) -> bool {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "function_declaration" {
            return true;
        }
        parent = p.parent();
    }
    false
}
