//! AST link model.
//! Represents a Markdown link extracted from documentation.

use serde::Deserialize;
use serde::Serialize;
use vibe_ast::nodes::link_node::LinkNode;

use crate::models::ast_model::AstModel;

/// Markdown link with display text and URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstLinkModel {
    /// Visible text of the link.
    pub text: String,
    /// Target URL.
    pub url: String,
}

/// Converts a parsed link node into the AST link model.
impl From<LinkNode> for AstLinkModel {
    fn from(node: LinkNode) -> Self {
        AstLinkModel {
            text: node.text,
            url: node.url,
        }
    }
}

/// AST model implementation for link declarations.
impl AstModel for AstLinkModel {
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
