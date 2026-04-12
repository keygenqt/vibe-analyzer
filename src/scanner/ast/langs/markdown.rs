//! Markdown language parser using tree-sitter.
//! Extracts frontmatter, headings with previews, links, and code block languages.

use std::collections::HashMap;

use anyhow::Result;
use regex::Regex;
use tree_sitter::Node;
use tree_sitter::Parser;

use crate::scanner::ast::data::ast_data::AstData;
use crate::scanner::ast::data::ast_data_heading::AstHeading;
use crate::scanner::ast::data::ast_data_links::AstLink;
use crate::scanner::ast::traits::LanguageParser;

/// Parser for Markdown files.
#[derive(Default)]
pub struct MarkdownParser;

impl LanguageParser for MarkdownParser {
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut data = AstData::default();

        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        if i < lines.len() && lines[i].trim() == "---" {
            i += 1;
            let mut frontmatter_map = HashMap::new();
            while i < lines.len() && lines[i].trim() != "---" {
                if let Some((key, value)) = lines[i].split_once(':') {
                    let key = key.trim().to_string();
                    let value = value.trim();

                    if value.starts_with('[') && value.ends_with(']') {
                        // Массив — соединяем в строку через запятую
                        let items: String = value[1..value.len() - 1]
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect::<Vec<_>>()
                            .join(", ");
                        frontmatter_map.insert(key, items);
                    } else {
                        frontmatter_map.insert(key, value.to_string());
                    }
                }
                i += 1;
            }
            if !frontmatter_map.is_empty() {
                data.frontmatter = Some(frontmatter_map);
            }
        }

        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_markdown_fork::language())?;

        let tree = match parser.parse(content, None) {
            Some(tree) => tree,
            None => return Ok(AstData::default()),
        };

        let root_node = tree.root_node();

        let mut headings_map: HashMap<usize, (u8, String)> = HashMap::new();
        collect_headings(&root_node, content, &mut headings_map);

        let mut positions: Vec<_> = headings_map.keys().cloned().collect();
        positions.sort();

        for pos in positions {
            if let Some((level, title)) = headings_map.get(&pos) {
                let preview = extract_preview(content, pos);
                data.headings.push(AstHeading {
                    level: *level,
                    title: title.clone(),
                    preview,
                });
            }
        }

        visit_node(&root_node, content, &mut data);

        data.links.sort_by(|a, b| a.url.cmp(&b.url));
        data.links.dedup_by(|a, b| a.url == b.url);
        data.code_blocks.sort();
        data.code_blocks.dedup();

        Ok(data)
    }
}

/// Extracts links and code block languages from AST.
fn visit_node(node: &Node, content: &str, data: &mut AstData) {
    match node.kind() {
        "link" => {
            let text = node
                .children(&mut node.walk())
                .find(|c| c.kind() == "link_text")
                .and_then(|c| c.utf8_text(content.as_bytes()).ok())
                .map(|t| t.to_string())
                .unwrap_or_default();

            let url = node
                .children(&mut node.walk())
                .find(|c| c.kind() == "link_destination")
                .and_then(|c| c.utf8_text(content.as_bytes()).ok())
                .map(|u| u.to_string())
                .unwrap_or_default();

            if !text.is_empty() && !url.is_empty() && !text.contains(':') {
                data.links.push(AstLink { text, url });
            }
        }

        "fenced_code_block" => {
            let lang = node
                .children(&mut node.walk())
                .find(|c| c.kind() == "info_string")
                .and_then(|c| c.utf8_text(content.as_bytes()).ok())
                .map(|l| l.trim().to_string())
                .unwrap_or_default();

            if !lang.is_empty() {
                data.code_blocks.push(lang);
            }
        }

        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_node(&child, content, data);
    }
}

/// Collects headings with their byte positions.
fn collect_headings(node: &Node, content: &str, headings: &mut HashMap<usize, (u8, String)>) {
    if node.kind() == "atx_heading" {
        let level = node
            .children(&mut node.walk())
            .find(|c| {
                c.kind() == "atx_h1_marker"
                    || c.kind() == "atx_h2_marker"
                    || c.kind() == "atx_h3_marker"
                    || c.kind() == "atx_h4_marker"
                    || c.kind() == "atx_h5_marker"
                    || c.kind() == "atx_h6_marker"
            })
            .and_then(|m| m.utf8_text(content.as_bytes()).ok())
            .map(|m| m.len() as u8)
            .unwrap_or(1);

        let title = node
            .children(&mut node.walk())
            .find(|c| c.kind() == "heading_content")
            .and_then(|c| c.utf8_text(content.as_bytes()).ok())
            .map(|t| t.trim().to_string())
            .unwrap_or_default();

        if !title.is_empty() {
            headings.insert(node.end_byte(), (level, title));
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_headings(&child, content, headings);
    }
}

/// Extracts first non-empty line after heading as preview.
fn extract_preview(content: &str, start_byte: usize) -> String {
    let after_heading = &content[start_byte..];
    let mut in_code_block = false;

    for line in after_heading.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if trimmed.starts_with("```") {
            if !in_code_block {
                in_code_block = true;
                continue;
            } else {
                in_code_block = false;
                continue;
            }
        }

        if in_code_block {
            return clean_preview(line);
        }

        return clean_preview(trimmed);
    }

    String::new()
}

/// Cleans markdown formatting from preview text.
fn clean_preview(text: &str) -> String {
    if text.contains('|') {
        let parts: Vec<&str> = text.split('|').collect();
        let cleaned_parts: Vec<String> = parts
            .iter()
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .map(|p| p.to_string())
            .collect();

        let result = cleaned_parts.join(" | ");
        return format!("| {} |", result);
    }

    let re_list = Regex::new(r"^[\s]*[-*+][\s]*").unwrap();
    let text = re_list.replace(text, "").to_string();

    let re_format = Regex::new(r"\*\*|__|\*|_|~~|`").unwrap();
    let text = re_format.replace_all(&text, "").to_string();

    let text = text
        .trim()
        .trim_end_matches(|c| c == '.' || c == ':' || c == ';' || c == ',' || c == '!')
        .trim();

    let re_spaces = Regex::new(r"\s+").unwrap();
    re_spaces.replace_all(text, " ").to_string()
}
