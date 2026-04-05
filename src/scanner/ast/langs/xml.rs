//! XML language AST parser
//!
//! This module provides tree-sitter based AST parsing for XML files,
//! extracting element tags, attributes, and comments.

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{parser::AstData, traits::LanguageParser};

/// Parser for XML language files
#[derive(Default)]
pub struct XmlParser;

impl LanguageParser for XmlParser {
    /// Parses XML source code and extracts AST data
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_xml::LANGUAGE_XML.into())?;

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

/// Recursively traverses the AST and extracts relevant nodes
fn visit_node(node: &Node, content: &str, data: &mut AstData) {
    let kind = node.kind();

    match kind {
        // Comments
        "comment" => {
            if let Ok(comment) = node.utf8_text(content.as_bytes()) {
                let cleaned = comment.trim().to_string();
                if !cleaned.is_empty() {
                    data.comments.push(cleaned);
                }
            }
        }

        // XML elements
        "element" => {
            if let Some(name) = extract_tag_name(node, content) {
                data.tags.push(name);
            }
            extract_attributes_from_element(node, content, data);
        }

        // XML attributes
        "Attribute" => {
            if let Some(attr) = extract_attribute_name(node, content) {
                data.attributes.push(attr);
            }
        }

        _ => {}
    }

    // Recursively process children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_node(&child, content, data);
    }
}

/// Extracts tag name from an XML element node
fn extract_tag_name(node: &Node, content: &str) -> Option<String> {
    // Try structured tree-sitter node first
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "tag_name" {
            if let Ok(name) = child.utf8_text(content.as_bytes()) {
                let cleaned = name.trim().to_string();
                if !cleaned.is_empty() {
                    return Some(cleaned);
                }
            }
        }
    }

    // Fallback: try Name nodes (for older grammar versions)
    let mut cursor2 = node.walk();
    for child in node.children(&mut cursor2) {
        if child.kind() == "Name" {
            if let Ok(name) = child.utf8_text(content.as_bytes()) {
                let cleaned = name.trim().to_string();
                if !cleaned.is_empty() && !cleaned.starts_with('?') && !cleaned.starts_with('!') {
                    return Some(cleaned);
                }
            }
        }
    }

    // Fallback: manual parsing from text (for malformed or unusual cases)
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        if let Some(start) = text.find('<') {
            let after_start = &text[start + 1..];
            if let Some(end) = after_start.find(|c| c == ' ' || c == '>' || c == '/') {
                let tag_name = after_start[..end].trim();
                if !tag_name.is_empty() && !tag_name.starts_with('?') && !tag_name.starts_with('!')
                {
                    return Some(tag_name.to_string());
                }
            }
        }
    }

    None
}

/// Extracts all attributes from an XML element
fn extract_attributes_from_element(node: &Node, content: &str, data: &mut AstData) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "Attribute" {
            if let Some(attr) = extract_attribute_name(&child, content) {
                data.attributes.push(attr);
            }
        }
    }
}

/// Extracts attribute name from an Attribute node
fn extract_attribute_name(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "Name" {
            if let Ok(name) = child.utf8_text(content.as_bytes()) {
                let cleaned = name.trim().to_string();
                if !cleaned.is_empty() {
                    return Some(cleaned);
                }
            }
        }
    }
    None
}

/// Sorts and deduplicates all extracted data
fn clean_data(data: &mut AstData) {
    let fields = [
        &mut data.comments,
        &mut data.doc_comments,
        &mut data.tags,
        &mut data.attributes,
    ];

    for field in fields {
        field.sort();
        field.dedup();
    }
}
