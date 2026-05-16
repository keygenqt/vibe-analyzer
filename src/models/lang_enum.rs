//! AST enum model.
//! Represents an enum declaration extracted from source code.

use serde::Deserialize;
use serde::Serialize;
use vibe_ast::nodes::enum_node::EnumNode;

use crate::analyzer::AnalyzeMode;
use crate::models::trait_format_model::TraitFormatModel;
use crate::utils::normalize::body_batch_normalize;

/// Enum declaration with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangEnumModel {
    /// Full enum signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
    /// Enum body from opening `{` to closing `}`
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<String>,
}

/// Converts a parsed enum node into the AST enum model.
impl From<EnumNode> for LangEnumModel {
    fn from(node: EnumNode) -> Self {
        LangEnumModel {
            signature: node.signature,
            comments: node.comments,
            body: node.body,
        }
    }
}

/// AST model implementation for enum declarations.
impl TraitFormatModel for LangEnumModel {
    fn format_ast(&self) -> Self {
        self.clone()
    }

    fn format_analyze(&self) -> Self {
        self.format_ast()
    }

    fn format_index(&self) -> Self {
        Self {
            signature: self.signature.clone(),
            comments: self.comments.clone(),
            body: None,
        }
    }

    fn format_tool(&self) -> Self {
        self.format_index()
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
