use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{parser::AstData, traits::LanguageParser};

#[derive(Default)]
pub struct BashParser;

impl LanguageParser for BashParser {
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_bash::LANGUAGE.into())?;

        let tree = match parser.parse(content, None) {
            Some(tree) => tree,
            None => return Ok(AstData::default()),
        };

        let root_node = tree.root_node();
        let mut data = AstData::default();

        visit_node(&root_node, content, &mut data);
        clean_data(&mut data);

        Ok(data)
    }
}

fn visit_node(node: &Node, content: &str, data: &mut AstData) {
    let kind = node.kind();

    match kind {
        "comment" => {
            if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                let cleaned = comment.trim().to_string();
                if !cleaned.is_empty() {
                    if cleaned.starts_with("##") || cleaned.starts_with("###") {
                        data.doc_comments.push(cleaned);
                    } else {
                        data.comments.push(cleaned);
                    }
                }
            }
        }
        "variable_assignment" => {
            if let Some(name) = extract_variable_name(node, content) {
                data.variables.push(name);
            }
        }
        "function_definition" => {
            if let Some(name) = extract_function_name(node, content) {
                data.functions.push(name);
            }
        }
        "source_command" => {
            if let Some(path) = extract_source_path(node, content) {
                data.imports.push(path);
            }
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_node(&child, content, data);
    }
}

fn extract_variable_name(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        if kind == "variable_name" {
            if let Ok(name) = child.utf8_text(content.as_bytes()) {
                let cleaned = name.trim().to_string();
                if !cleaned.is_empty() && !cleaned.starts_with('#') {
                    return Some(cleaned);
                }
            }
        }
    }
    None
}

fn extract_function_name(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        if kind == "word" || kind == "identifier" || kind == "name" {
            if let Ok(name) = child.utf8_text(content.as_bytes()) {
                if !name.is_empty() && name != "function" && !name.contains(' ') {
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}

fn extract_source_path(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        if kind == "word" || kind == "string" || kind == "raw_string" {
            if let Ok(path) = child.utf8_text(content.as_bytes()) {
                let cleaned = path
                    .replace("source ", "")
                    .replace(".", "")
                    .trim()
                    .to_string();
                if !cleaned.is_empty() && !cleaned.starts_with('#') {
                    return Some(cleaned);
                }
            }
        }
    }
    None
}

fn clean_data(data: &mut AstData) {
    let fields = [
        &mut data.functions,
        &mut data.imports,
        &mut data.comments,
        &mut data.doc_comments,
        &mut data.variables,
    ];
    for field in fields {
        field.sort();
        field.dedup();
    }
}
