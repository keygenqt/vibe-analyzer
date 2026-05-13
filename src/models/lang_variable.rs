//! AST variable model.
//! Represents a global variable or constant declaration extracted from source code.

use serde::Deserialize;
use serde::Serialize;
use vibe_ast::nodes::variable_node::VariableNode;

use crate::analyzer::AnalyzeMode;
use crate::models::trait_format_model::TraitFormatModel;

/// Global variable or constant with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangVariableModel {
    /// Full variable signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
}

/// Converts a parsed variable node into the AST variable model.
impl From<VariableNode> for LangVariableModel {
    fn from(node: VariableNode) -> Self {
        LangVariableModel {
            signature: node.signature,
            comments: node.comments,
        }
    }
}

/// AST model implementation for variable declarations.
impl TraitFormatModel for LangVariableModel {
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
            AnalyzeMode::Advice => Self {
                signature: self.signature.clone(),
                comments: vec![],
            },
            _ => self.clone(),
        }
    }
}
