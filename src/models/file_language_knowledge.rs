//! File language knowledge model.
//! Markdown structures: headings, links, code blocks, and frontmatter.

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use crate::analyzer::AnalyzeMode;
use crate::models::lang_heading::LangHeadingModel;
use crate::models::lang_link::LangLinkModel;
use crate::models::trait_format_model::TraitFormatModel;

/// Knowledge base structures — headings, links, and metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileLanguageKnowledgeModel {
    /// Markdown headings with previews.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub headings: Vec<LangHeadingModel>,
    /// Markdown links.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub links: Vec<LangLinkModel>,
    /// Code block languages used in the file.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub code_blocks: Vec<String>,
    /// YAML frontmatter key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub frontmatter: Option<HashMap<String, String>>,
}

/// Format implementation for file language knowledge model.
impl TraitFormatModel for FileLanguageKnowledgeModel {
    fn format_ast(&self) -> Self {
        Self {
            headings: self.headings.iter().map(|h| h.format_ast()).collect(),
            links: self.links.iter().map(|l| l.format_ast()).collect(),
            code_blocks: self.code_blocks.clone(),
            frontmatter: self.frontmatter.clone(),
        }
    }

    fn format_analyze(&self) -> Self {
        Self {
            headings: self.headings.iter().map(|h| h.format_analyze()).collect(),
            links: self.links.iter().map(|l| l.format_analyze()).collect(),
            code_blocks: self.code_blocks.clone(),
            frontmatter: self.frontmatter.clone(),
        }
    }

    fn format_index(&self) -> Self {
        Self {
            headings: self.headings.iter().map(|h| h.format_index()).collect(),
            links: self.links.iter().map(|l| l.format_index()).collect(),
            code_blocks: self.code_blocks.clone(),
            frontmatter: self.frontmatter.clone(),
        }
    }

    fn format_tool(&self) -> Self {
        Self {
            headings: self.headings.iter().map(|h| h.format_tool()).collect(),
            links: self.links.iter().map(|l| l.format_tool()).collect(),
            code_blocks: self.code_blocks.clone(),
            frontmatter: self.frontmatter.clone(),
        }
    }

    fn format_batch(&self, mode: AnalyzeMode) -> Self {
        match mode {
            AnalyzeMode::Meta => Self {
                headings: self.headings.iter().map(|h| h.format_batch(mode)).collect(),
                links: self.links.iter().map(|l| l.format_batch(mode)).collect(),
                code_blocks: self.code_blocks.clone(),
                frontmatter: self.frontmatter.clone(),
            },
            _ => Self {
                headings: self.headings.iter().map(|h| h.format_batch(mode)).collect(),
                links: self.links.iter().map(|l| l.format_batch(mode)).collect(),
                code_blocks: vec![],
                frontmatter: None,
            },
        }
    }
}
