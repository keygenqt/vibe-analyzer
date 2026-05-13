//! AST class model.
//! Represents a class declaration extracted from source code.

use serde::Deserialize;
use serde::Serialize;
use vibe_ast::nodes::class_node::ClassNode;

use crate::analyzer::AnalyzeMode;
use crate::models::trait_format_model::TraitFormatModel;
use crate::utils::normalize::body_batch_normalize;

/// Class declaration with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangClassModel {
    /// Full class signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
    /// Class body from opening `{` to closing `}`
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<String>,
}

/// Converts a parsed class node into the AST class model.
impl From<ClassNode> for LangClassModel {
    fn from(node: ClassNode) -> Self {
        LangClassModel {
            signature: node.signature,
            comments: node.comments,
            body: node.body,
        }
    }
}

/// AST model implementation for class declarations.
impl TraitFormatModel for LangClassModel {
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
