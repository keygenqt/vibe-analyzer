//! AST interface model.
//! Represents an interface or trait declaration extracted from source code.

use serde::Deserialize;
use serde::Serialize;
use vibe_ast::nodes::interface_node::InterfaceNode;

use crate::analyzer::AnalyzeMode;
use crate::models::trait_format_model::TraitFormatModel;
use crate::utils::normalize::body_batch_normalize;

/// Interface or trait declaration with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangInterfaceModel {
    /// Full interface signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
    /// Interface body from opening `{` to closing `}`
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<String>,
}

/// Converts a parsed interface node into the AST interface model.
impl From<InterfaceNode> for LangInterfaceModel {
    fn from(node: InterfaceNode) -> Self {
        LangInterfaceModel {
            signature: node.signature,
            comments: node.comments,
            body: node.body,
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

    fn format_batch(&self, mode: AnalyzeMode) -> Self {
        match mode {
            AnalyzeMode::Meta => Self {
                signature: self.signature.clone(),
                comments: self.comments.clone(),
                body: None,
            },
            AnalyzeMode::Advice => Self {
                signature: self.signature.clone(),
                comments: vec![],
                body: body_batch_normalize(&self.body),
            },
            AnalyzeMode::Debt | AnalyzeMode::Errors => Self {
                signature: self.signature.clone(),
                comments: self.comments.clone(),
                body: body_batch_normalize(&self.body),
            },
            AnalyzeMode::Ast => self.clone(),
        }
    }
}
