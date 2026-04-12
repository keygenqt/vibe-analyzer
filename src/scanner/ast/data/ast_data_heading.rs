//! AST data structures for Markdown documents.
//! Stores frontmatter, headings, links, and code block languages.

use serde::Deserialize;
use serde::Serialize;

/// Markdown heading structure with level and text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstHeading {
    pub level: u8,
    pub title: String,
    pub preview: String,
}
