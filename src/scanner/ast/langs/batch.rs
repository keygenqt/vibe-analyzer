use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{parser::AstData, traits::LanguageParser};

#[derive(Default)]
pub struct BatchParser;

impl LanguageParser for BatchParser {
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_batch::LANGUAGE.into())?;

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
        "variable" => {
            if let Ok(var_name) = node.utf8_text(content.as_bytes()) {
                let clean = var_name.trim().to_string();
                if !clean.is_empty() {
                    data.variables.push(clean);
                }
            }
        }
        "label" => {
            if let Ok(label) = node.utf8_text(content.as_bytes()) {
                let clean = label.trim_start_matches(':').trim().to_string();
                if !clean.is_empty() {
                    data.functions.push(clean);
                }
            }
        }
        "command" => {
            if let Ok(cmd) = node.utf8_text(content.as_bytes()) {
                let clean = cmd.trim().to_string();
                if !clean.is_empty() {
                    data.imports.push(clean);
                }
            }
        }
        "comment" => {
            if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                let clean = comment.trim().to_string();
                if !clean.is_empty() {
                    data.comments.push(clean);
                }
            }
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_node(&child, content, data);
    }
}

fn clean_data(data: &mut AstData) {
    let fields = [
        &mut data.functions,
        &mut data.variables,
        &mut data.comments,
        &mut data.imports,
    ];
    for field in fields {
        field.sort();
        field.dedup();
    }
}
