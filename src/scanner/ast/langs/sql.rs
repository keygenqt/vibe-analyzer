use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{parser::AstData, traits::LanguageParser};

#[derive(Default)]
pub struct SqlParser;

impl LanguageParser for SqlParser {
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_postgres::LANGUAGE.into())?;

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
                    data.comments.push(cleaned);
                }
            }
        }
        "columnDef" => {
            if let Some(name_node) = node.child(0) {
                if let Ok(name) = name_node.utf8_text(content.as_bytes()) {
                    let cleaned = name.trim().to_string();
                    if !is_sql_keyword(&cleaned.to_uppercase()) {
                        data.variables.push(cleaned);
                    }
                }
            }
        }
        "RelationExpr" | "qualified_name" | "any_name" => {
            if let Some(name) = extract_name(node, content) {
                if !is_sql_keyword(&name.to_uppercase()) {
                    let stmt_text = get_stmt_text(node, content).to_uppercase();

                    if stmt_text.contains("INDEX") {
                        data.structs.push(format!("[INDEX] {}", name));
                    } else {
                        data.tables.push(name);
                    }
                }
            }
        }
        "IndexName" | "opt_single_name" => {
            if let Some(name) = extract_name(node, content) {
                if !is_sql_keyword(&name.to_uppercase()) {
                    data.structs.push(format!("[INDEX] {}", name));
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

fn get_stmt_text(node: &Node, content: &str) -> String {
    let mut current = node.parent();
    while let Some(parent) = current {
        let kind = parent.kind();
        if kind.contains("Stmt")
            || kind == "drop_statement"
            || kind == "create_statement"
            || kind == "alter_statement"
            || kind == "select_statement"
            || kind == "insert_statement"
            || kind == "update_statement"
            || kind == "delete_statement"
        {
            if let Ok(text) = parent.utf8_text(content.as_bytes()) {
                return text.to_string();
            }
        }
        current = parent.parent();
    }
    String::new()
}

fn extract_name(node: &Node, content: &str) -> Option<String> {
    let kind = node.kind();
    if kind == "identifier" || kind == "ColId" || kind == "any_name" {
        if let Ok(text) = node.utf8_text(content.as_bytes()) {
            let cleaned = text.trim().replace('"', "");
            if !cleaned.is_empty() && !is_sql_keyword(&cleaned) {
                return Some(cleaned);
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(name) = extract_name(&child, content) {
            return Some(name);
        }
    }
    None
}

fn is_sql_keyword(name: &str) -> bool {
    let kw = name.to_uppercase();
    matches!(
        kw.as_str(),
        "SERIAL"
            | "INTEGER"
            | "TEXT"
            | "VARCHAR"
            | "TIMESTAMP"
            | "TABLE"
            | "INDEX"
            | "PRIMARY"
            | "KEY"
            | "NOT"
            | "NULL"
            | "IF"
            | "EXISTS"
            | "UNIQUE"
            | "DEFAULT"
            | "REFERENCES"
    )
}

fn clean_data(data: &mut AstData) {
    let fields = [
        &mut data.functions,
        &mut data.tables,
        &mut data.views,
        &mut data.variables,
        &mut data.comments,
        &mut data.structs,
    ];
    for field in fields {
        field.sort();
        field.dedup();
    }
}
