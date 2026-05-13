//! AST heading model.
//! Represents a Markdown heading extracted from documentation.

use serde::Deserialize;
use serde::Serialize;
use vibe_ast::nodes::heading_node::HeadingNode;

use crate::analyzer::AnalyzeMode;
use crate::models::trait_format_model::TraitFormatModel;

/// Markdown heading with level, title, and content preview.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangHeadingModel {
    /// Heading level (1-6).
    pub level: u8,
    /// Heading text.
    pub title: String,
    /// First non-empty line after the heading.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub preview: Option<String>,
    /// Heading body from title to next heading of same or higher level.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<String>,
}

/// Converts a parsed heading node into the AST heading model.
impl From<HeadingNode> for LangHeadingModel {
    fn from(node: HeadingNode) -> Self {
        let preview = node.body.as_ref().and_then(|b| {
            b.lines()
                .find(|l| {
                    let trimmed = l.trim();
                    !trimmed.is_empty() && !trimmed.starts_with("```")
                })
                .map(|l| l.trim().split_whitespace().collect::<Vec<_>>().join(" "))
        });
        LangHeadingModel {
            level: node.level,
            title: node.title,
            preview,
            body: node.body,
        }
    }
}

/// AST model implementation for heading declarations.
impl TraitFormatModel for LangHeadingModel {
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
                level: self.level,
                title: self.title.clone(),
                preview: None,
                body: None,
            },
            AnalyzeMode::Advice | AnalyzeMode::Debt | AnalyzeMode::Errors => Self {
                level: self.level,
                title: self.title.clone(),
                preview: self.preview.clone(),
                body: self.body.clone(),
            },
            AnalyzeMode::Ast => self.clone(),
        }
    }
}
