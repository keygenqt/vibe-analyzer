//! AST struct model.
//! Represents a struct or record declaration extracted from source code.

use serde::{Deserialize, Serialize};
use vibe_ast::nodes::struct_node::StructNode;

use crate::models::ast_model::AstModel;

/// Struct or record declaration with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstStructModel {
    /// Full struct signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
}

impl From<StructNode> for AstStructModel {
    fn from(node: StructNode) -> Self {
        AstStructModel {
            signature: node.signature,
            comments: node.comments,
        }
    }
}

impl AstModel for AstStructModel {
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
