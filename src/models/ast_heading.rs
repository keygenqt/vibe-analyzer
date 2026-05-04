//! AST heading model.
//! Represents a Markdown heading extracted from documentation.

use serde::{Deserialize, Serialize};
use vibe_ast::nodes::heading_node::HeadingNode;

use crate::models::ast_model::AstModel;

/// Markdown heading with level, title, and content preview.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstHeadingModel {
    /// Heading level (1-6).
    pub level: u8,
    /// Heading text.
    pub title: String,
    /// First non-empty line after the heading.
    pub preview: String,
}

impl From<HeadingNode> for AstHeadingModel {
    fn from(node: HeadingNode) -> Self {
        AstHeadingModel {
            level: node.level,
            title: node.title,
            preview: node.preview,
        }
    }
}

impl AstModel for AstHeadingModel {
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
