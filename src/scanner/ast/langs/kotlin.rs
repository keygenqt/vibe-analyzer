//! Kotlin language AST parser

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{
    data::{
        ast_data::AstData, ast_data_class::AstDataClass, ast_data_enum::AstDataEnum,
        ast_data_function::AstDataFunction, ast_data_interface::AstDataInterface,
        ast_data_variable::AstDataVariable,
    },
    traits::LanguageParser,
};

/// Parser for Kotlin language files
#[derive(Default)]
pub struct KotlinParser;

impl LanguageParser for KotlinParser {
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        let lang = tree_sitter_kotlin::language();
        parser.set_language(&unsafe { std::mem::transmute(lang) })?;

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

/// Recursively traverses the AST and extracts relevant nodes
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
        // Module comments
        "multiline_comment" => {
            let is_at_root = node.parent().map_or(false, |p| p.kind() == "source_file");
            let is_first = is_module_comment(node, content);

            if is_at_root && is_first {
                if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                    let cleaned = comment.trim();
                    if cleaned.starts_with("/**") {
                        let body = cleaned
                            .trim_start_matches("/**")
                            .trim_end_matches("*/")
                            .trim();

                        let lines: Vec<String> = body
                            .lines()
                            .map(|line| line.trim().trim_start_matches('*').trim().to_string())
                            .filter(|line| !line.is_empty())
                            .collect();

                        let comment_text = lines.join(" ").trim().to_string();
                        if !comment_text.is_empty() && !data.comments.contains(&comment_text) {
                            data.comments.push(comment_text);
                        }
                    }
                }
            }
        }

        // Top-level functions
        "function_declaration" => {
            if !is_inside_class(node) {
                if let Some(signature) = extract_kotlin_function_signature(node, content) {
                    functions.push(AstDataFunction {
                        signature,
                        comments: extract_kotlin_doc_comments(node, content),
                    });
                }
            }
        }

        // Classes, enums, interfaces
        "class_declaration" => {
            if !is_inside_class(node) {
                if let Ok(text) = node.utf8_text(content.as_bytes()) {
                    let trimmed = text.trim();

                    if trimmed.starts_with("sealed class") {
                        // Skip sealed classes
                    } else if trimmed.starts_with("enum ") || trimmed.starts_with("enum class") {
                        if let Some(signature) = extract_kotlin_class_signature(node, content) {
                            enums.push(AstDataEnum {
                                signature,
                                comments: extract_kotlin_doc_comments(node, content),
                            });
                        }
                    } else if trimmed.starts_with("interface ") {
                        if let Some(signature) = extract_kotlin_class_signature(node, content) {
                            interfaces.push(AstDataInterface {
                                signature,
                                comments: extract_kotlin_doc_comments(node, content),
                            });
                        }
                    } else {
                        if let Some(signature) = extract_kotlin_class_signature(node, content) {
                            classes.push(AstDataClass {
                                signature,
                                comments: extract_kotlin_doc_comments(node, content),
                            });
                        }
                    }
                }
            }
        }

        // Global properties
        "property_declaration" => {
            if !is_inside_class(node) && !is_inside_function(node) {
                if let Some(signature) = extract_kotlin_property_signature(node, content) {
                    variables.push(AstDataVariable {
                        signature,
                        comments: extract_kotlin_doc_comments(node, content),
                    });
                }
            }
        }

        // Imports
        "import_header" => {
            if let Ok(text) = node.utf8_text(content.as_bytes()) {
                let cleaned = text.replace("import ", "").trim().to_string();
                let first_line = cleaned.lines().next().unwrap_or(&cleaned).to_string();
                if !first_line.is_empty() && !data.imports.contains(&first_line) {
                    data.imports.push(first_line);
                }
            }
        }

        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_node(
            &child, content, data, functions, classes, enums, interfaces, variables,
        );
    }
}

/// Extracts function signature without body
fn extract_kotlin_function_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let first_line = text.lines().next().unwrap_or(text);
        if let Some(pos) = first_line.find('{') {
            return Some(first_line[..pos].trim().to_string());
        }
        if let Some(pos) = first_line.find('=') {
            return Some(first_line[..pos].trim().to_string());
        }
        Some(first_line.trim().to_string())
    } else {
        None
    }
}

/// Extracts class signature without body
fn extract_kotlin_class_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let first_line = text.lines().next().unwrap_or(text);
        let cut_pos = first_line
            .find('{')
            .or_else(|| first_line.find('('))
            .unwrap_or(first_line.len());
        let signature = first_line[..cut_pos].trim().to_string();
        if !signature.is_empty() {
            return Some(signature);
        }
    }
    None
}

/// Extracts property signature without value
fn extract_kotlin_property_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let signature = text.split('=').next().unwrap_or(text).trim().to_string();
        if !signature.is_empty() && signature.len() < 100 {
            return Some(signature);
        }
    }
    None
}

/// Extracts doc comments before a node
fn extract_kotlin_doc_comments(node: &Node, content: &str) -> Vec<String> {
    let start_byte = node.range().start_byte;
    let parent_start = node.parent().map(|p| p.range().start_byte).unwrap_or(0);
    let prefix_text = &content[parent_start..start_byte];

    if let Some(open_idx) = prefix_text.rfind("/**") {
        let potential_doc = &prefix_text[open_idx..];
        if let Some(close_idx) = potential_doc.find("*/") {
            let kdoc_text = &potential_doc[..close_idx + 2];
            let between = &potential_doc[close_idx + 2..].trim();

            let is_clean_gap = between.lines().all(|line| {
                let l = line.trim();
                l.is_empty() || l.starts_with("//")
            });

            if is_clean_gap {
                let body = kdoc_text
                    .trim_start_matches("/**")
                    .trim_end_matches("*/")
                    .trim();
                return body
                    .lines()
                    .map(|line| line.trim().trim_start_matches('*').trim().to_string())
                    .filter(|line| !line.is_empty())
                    .collect();
            }
        }
    }
    Vec::new()
}

/// Checks if a comment is a module-level comment
fn is_module_comment(node: &Node, _content: &str) -> bool {
    node.prev_sibling().is_none() && node.parent().map_or(false, |p| p.kind() == "source_file")
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
