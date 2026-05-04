//! AST enum model.
//! Represents an enum declaration extracted from source code.

use serde::{Deserialize, Serialize};
use vibe_ast::nodes::enum_node::EnumNode;

use crate::models::ast_model::AstModel;

/// Enum declaration with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstEnumModel {
    /// Full enum signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
}

impl From<EnumNode> for AstEnumModel {
    fn from(node: EnumNode) -> Self {
        AstEnumModel {
            signature: node.signature,
            comments: node.comments,
        }
    }
}

impl AstModel for AstEnumModel {
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
