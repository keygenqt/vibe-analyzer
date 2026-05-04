//! AST variable model.
//! Represents a global variable or constant declaration extracted from source code.

use serde::{Deserialize, Serialize};
use vibe_ast::nodes::variable_node::VariableNode;

use crate::models::ast_model::AstModel;

/// Global variable or constant with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstVariableModel {
    /// Full variable signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
}

impl From<VariableNode> for AstVariableModel {
    fn from(node: VariableNode) -> Self {
        AstVariableModel {
            signature: node.signature,
            comments: node.comments,
        }
    }
}

impl AstModel for AstVariableModel {
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
