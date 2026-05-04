//! Response structure for the show_stats tool.

use std::convert::TryFrom;

use anyhow::Result;
use serde::Serialize;

use crate::{
    mcp::responses::McpResponse,
    opensearch::tools::query::show_stats_search::{LanguageStatsOutput, ShowStatsOutput},
};

/// Response from show_stats tool.
#[derive(Debug, Serialize)]
pub struct ShowStatsResponse {
    /// Stats per language.
    pub languages: Vec<ShowStatLanguageResponse>,
    /// Total across all languages.
    pub total: ShowStatTotalResponse,
}

/// Statistics for a single language.
#[derive(Debug, Serialize)]
pub struct ShowStatLanguageResponse {
    /// Language name.
    pub language: String,
    /// File count.
    pub files: usize,
    /// Total lines of code.
    pub lines: usize,
    /// AST objects count.
    pub ast_objects: usize,
    /// Human-readable size.
    pub size_human: String,
}

/// Total statistics across all languages.
#[derive(Debug, Serialize)]
pub struct ShowStatTotalResponse {
    /// Total files.
    pub files: usize,
    /// Total lines.
    pub lines: usize,
    /// Total AST objects.
    pub ast_objects: usize,
    /// Total size.
    pub size_human: String,
}

impl From<LanguageStatsOutput> for ShowStatLanguageResponse {
    fn from(l: LanguageStatsOutput) -> Self {
        ShowStatLanguageResponse {
            language: l.language,
            files: l.files,
            lines: l.lines,
            ast_objects: l.ast_objects,
            size_human: l.size_human,
        }
    }
}

impl From<LanguageStatsOutput> for ShowStatTotalResponse {
    fn from(l: LanguageStatsOutput) -> Self {
        ShowStatTotalResponse {
            files: l.files,
            lines: l.lines,
            ast_objects: l.ast_objects,
            size_human: l.size_human,
        }
    }
}

impl TryFrom<ShowStatsOutput> for ShowStatsResponse {
    type Error = anyhow::Error;

    fn try_from(output: ShowStatsOutput) -> Result<Self> {
        Ok(ShowStatsResponse {
            languages: output.languages.into_iter().map(Into::into).collect(),
            total: output.total.into(),
        })
    }
}

impl McpResponse for ShowStatsResponse {
    fn format(&self) -> String {
        format!(
            "[{} language(s), {} file(s), {} line(s)]\nUse this data to respond to the user.\n\n{}",
            self.languages.len(),
            self.total.files,
            self.total.lines,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
