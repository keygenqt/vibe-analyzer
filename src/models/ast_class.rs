//! AST class model.
//! Represents a class declaration extracted from source code.

use serde::Deserialize;
use serde::Serialize;
use vibe_ast::nodes::class_node::ClassNode;

use crate::models::ast_model::AstModel;

/// Class declaration with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstClassModel {
    /// Full class signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
}

/// Converts a parsed class node into the AST class model.
impl From<ClassNode> for AstClassModel {
    fn from(node: ClassNode) -> Self {
        AstClassModel {
            signature: node.signature,
            comments: node.comments,
        }
    }
}

/// AST model implementation for class declarations.
impl AstModel for AstClassModel {
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
