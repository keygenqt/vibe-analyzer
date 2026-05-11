//! AST interface model.
//! Represents an interface or trait declaration extracted from source code.

use serde::Deserialize;
use serde::Serialize;
use vibe_ast::nodes::interface_node::InterfaceNode;

use crate::models::trait_format_model::TraitFormatModel;

/// Interface or trait declaration with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangInterfaceModel {
    /// Full interface signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
}

/// Converts a parsed interface node into the AST interface model.
impl From<InterfaceNode> for LangInterfaceModel {
    fn from(node: InterfaceNode) -> Self {
        LangInterfaceModel {
            signature: node.signature,
            comments: node.comments,
        }
    }
}

/// AST model implementation for interface declarations.
impl TraitFormatModel for LangInterfaceModel {
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
