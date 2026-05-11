//! AST heading model.
//! Represents a Markdown heading extracted from documentation.

use serde::Deserialize;
use serde::Serialize;
use vibe_ast::nodes::heading_node::HeadingNode;

use crate::models::trait_format_model::TraitFormatModel;

/// Markdown heading with level, title, and content preview.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangHeadingModel {
    /// Heading level (1-6).
    pub level: u8,
    /// Heading text.
    pub title: String,
    /// First non-empty line after the heading.
    pub preview: String,
}

/// Converts a parsed heading node into the AST heading model.
impl From<HeadingNode> for LangHeadingModel {
    fn from(node: HeadingNode) -> Self {
        LangHeadingModel {
            level: node.level,
            title: node.title,
            preview: node.preview,
        }
    }
}

/// AST model implementation for heading declarations.
impl TraitFormatModel for LangHeadingModel {
    fn format_ast(&self) -> Self {
        self.clone()
    }

    fn format_analyze(&self) -> Self {
        self.format_ast()
    }

    fn format_index(&self) -> Self {
        self.format_ast()
    }

    fn format_tool(&self) -> Self {
        self.format_ast()
    }
}
