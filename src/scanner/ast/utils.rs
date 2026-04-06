//! Common AST extraction utilities

use tree_sitter::Node;

/// Extracts import path from various import statements
pub fn extract_import_path(node: &Node, content: &str, prefix: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let cleaned = text.replace(prefix, "").replace(";", "").trim().to_string();
        if !cleaned.is_empty() && cleaned.len() < 200 {
            return Some(cleaned);
        }
    }
    None
}

/// Extracts function/struct/variable signature without body
pub fn extract_signature(node: &Node, content: &str) -> Option<String> {
    let text = node.utf8_text(content.as_bytes()).ok()?;
    let first_line = text.lines().next()?.trim();

    let cut_pos = first_line
        .find('{')
        .or_else(|| first_line.find('='))
        .or_else(|| first_line.find(';'))
        .or_else(|| first_line.find(':'))
        .unwrap_or(first_line.len());

    Some(first_line[..cut_pos].trim().to_string())
}
