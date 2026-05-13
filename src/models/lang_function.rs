//! AST function model.
//! Represents a function or method declaration extracted from source code.

use serde::Deserialize;
use serde::Serialize;
use vibe_ast::nodes::function_node::FunctionNode;

use crate::analyzer::AnalyzeMode;
use crate::models::trait_format_model::TraitFormatModel;

/// Function or method signature with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangFunctionModel {
    /// Full function signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
    /// Function body from opening `{` to closing `}`
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<String>,
}

/// Converts a parsed function node into the AST function model.
impl From<FunctionNode> for LangFunctionModel {
    fn from(node: FunctionNode) -> Self {
        LangFunctionModel {
            signature: node.signature,
            comments: node.comments,
            body: node.body,
        }
    }
}

/// AST model implementation for function declarations.
impl TraitFormatModel for LangFunctionModel {
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
                body: self.body.clone(),
            },
            AnalyzeMode::Debt | AnalyzeMode::Errors => Self {
                signature: self.signature.clone(),
                comments: self.comments.clone(),
                body: self.body.clone(),
            },
            AnalyzeMode::Ast => self.clone(),
        }
    }
}
