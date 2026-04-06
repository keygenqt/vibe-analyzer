//! ArkTS (Ark TypeScript) language AST parser

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{
    data::{
        ast_data::AstData, ast_data_class::AstDataClass, ast_data_enum::AstDataEnum,
        ast_data_function::AstDataFunction, ast_data_interface::AstDataInterface,
        ast_data_struct::AstDataStruct, ast_data_variable::AstDataVariable,
    },
    traits::LanguageParser,
    utils::extract_signature,
};

/// Parser for ArkTS (HarmonyOS) language files
#[derive(Default)]
pub struct ArkTSParser;

impl LanguageParser for ArkTSParser {
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_arkts::LANGUAGE.into())?;

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

/// Recursively traverses the AST and extracts relevant nodes
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
        // Module comment at the beginning of file
        "comment" => {
            if !is_inside_function(node) && node.prev_sibling().is_none() {
                if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                    let cleaned = comment.trim().to_string();
                    if cleaned.starts_with("/**") {
                        let content = cleaned[3..cleaned.len() - 2].trim();
                        let lines: Vec<String> = content
                            .lines()
                            .map(|line| line.trim_start_matches('*').trim_start().to_string())
                            .collect();
                        let comment_text = lines.join(" ").trim().to_string();
                        if !comment_text.is_empty() {
                            data.comments.push(comment_text);
                        }
                    }
                }
            }
        }

        // Global variables (const, let)
        "variable_declaration" => {
            if let Some(signature) = extract_signature(node, content) {
                variables.push(AstDataVariable {
                    signature,
                    comments: extract_arkts_doc_comments(node, content),
                });
            }
        }

        // Functions
        "function_declaration" => {
            if let Some(signature) = extract_signature(node, content) {
                functions.push(AstDataFunction {
                    signature,
                    comments: extract_arkts_doc_comments(node, content),
                });
            }
        }

        // Structs (ArkTS components)
        "component_declaration" => {
            let mut name = String::new();
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "identifier" {
                    if let Ok(text) = child.utf8_text(content.as_bytes()) {
                        name = text.to_string();
                        break;
                    }
                }
            }
            let signature = format!("struct {}", name);
            structs.push(AstDataStruct {
                signature,
                comments: extract_arkts_doc_comments(node, content),
            });
        }

        // Classes
        "class_declaration" => {
            if let Some(signature) = extract_signature(node, content) {
                classes.push(AstDataClass {
                    signature,
                    comments: extract_arkts_doc_comments(node, content),
                });
            }
        }

        // Enums
        "enum_declaration" => {
            if let Some(signature) = extract_signature(node, content) {
                enums.push(AstDataEnum {
                    signature,
                    comments: extract_arkts_doc_comments(node, content),
                });
            }
        }

        // Interfaces
        "interface_declaration" => {
            if let Some(signature) = extract_signature(node, content) {
                interfaces.push(AstDataInterface {
                    signature,
                    comments: extract_arkts_doc_comments(node, content),
                });
            }
        }

        // Imports
        "import_declaration" => {
            if let Ok(text) = node.utf8_text(content.as_bytes()) {
                if let Some(start) = text.find("from '") {
                    let rest = &text[start + 6..];
                    if let Some(end) = rest.find('\'') {
                        let path = rest[..end].to_string();
                        if !path.is_empty() && path.len() < 200 {
                            data.imports.push(path);
                        }
                    }
                } else if let Some(start) = text.find("from \"") {
                    let rest = &text[start + 6..];
                    if let Some(end) = rest.find('"') {
                        let path = rest[..end].to_string();
                        if !path.is_empty() && path.len() < 200 {
                            data.imports.push(path);
                        }
                    }
                }
            }
        }

        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_node(
            &child, content, data, functions, structs, classes, enums, interfaces, variables,
        );
    }
}

/// Extracts doc comments (/** */) for ArkTS nodes, skipping export declarations
fn extract_arkts_doc_comments(node: &Node, content: &str) -> Vec<String> {
    let mut comments = Vec::new();

    let mut prev = node.prev_sibling();
    while let Some(sibling) = prev {
        let kind = sibling.kind();

        if kind == "export_declaration" || kind == "export" {
            prev = sibling.prev_sibling();
            continue;
        }

        if kind != "comment" {
            break;
        }

        if let Ok(text) = sibling.utf8_text(content.as_bytes()) {
            let cleaned = text.trim().to_string();
            if cleaned.starts_with("/**") {
                let content = cleaned[3..cleaned.len() - 2].trim();
                let lines: Vec<String> = content
                    .lines()
                    .map(|line| line.trim_start_matches('*').trim_start().to_string())
                    .collect();
                let comment_text = lines.join(" ").trim().to_string();
                if !comment_text.is_empty() {
                    comments.push(comment_text);
                }
            }
        }

        prev = sibling.prev_sibling();
    }

    if comments.is_empty() {
        if let Some(parent) = node.parent() {
            if parent.kind() == "export_declaration" {
                let mut prev = parent.prev_sibling();
                while let Some(sibling) = prev {
                    if sibling.kind() != "comment" {
                        break;
                    }
                    if let Ok(text) = sibling.utf8_text(content.as_bytes()) {
                        let cleaned = text.trim().to_string();
                        if cleaned.starts_with("/**") {
                            let content = cleaned[3..cleaned.len() - 2].trim();
                            let lines: Vec<String> = content
                                .lines()
                                .map(|line| line.trim_start_matches('*').trim_start().to_string())
                                .collect();
                            let comment_text = lines.join(" ").trim().to_string();
                            if !comment_text.is_empty() {
                                comments.push(comment_text);
                            }
                        }
                    }
                    prev = sibling.prev_sibling();
                }
            }
        }
    }

    comments.reverse();
    comments
}

/// Checks if a node is inside a function or method body
fn is_inside_function(node: &Node) -> bool {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "function_declaration" || p.kind() == "method_definition" {
            return true;
        }
        parent = p.parent();
    }
    false
}
