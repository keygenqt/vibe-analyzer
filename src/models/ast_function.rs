//! AST function model.
//! Represents a function or method declaration extracted from source code.

use serde::Deserialize;
use serde::Serialize;
use vibe_ast::nodes::function_node::FunctionNode;

use crate::models::ast_model::AstModel;

/// Function or method signature with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstFunctionModel {
    /// Full function signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
}

/// Converts a parsed function node into the AST function model.
impl From<FunctionNode> for AstFunctionModel {
    fn from(node: FunctionNode) -> Self {
        AstFunctionModel {
            signature: node.signature,
            comments: node.comments,
        }
    }
}

/// AST model implementation for function declarations.
impl AstModel for AstFunctionModel {
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
