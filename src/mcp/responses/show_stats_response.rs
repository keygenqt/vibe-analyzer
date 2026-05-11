//! Response structure for the show_stats tool.

use std::convert::TryFrom;

use anyhow::Result;
use serde::Serialize;

use crate::mcp::responses::McpResponse;
use crate::opensearch::tools::query::show_stats_search::LanguageStatsOutput;
use crate::opensearch::tools::query::show_stats_search::ShowStatsOutput;

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
    /// Number of lines in file.
    pub size_lines: usize,
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
    /// Number of lines in file.
    pub size_lines: usize,
    /// Total AST objects.
    pub ast_objects: usize,
    /// Total size.
    pub size_human: String,
}

/// Converts language stats to MCP response item.
impl From<LanguageStatsOutput> for ShowStatLanguageResponse {
    fn from(l: LanguageStatsOutput) -> Self {
        ShowStatLanguageResponse {
            language: l.language,
            files: l.files,
            size_lines: l.size_lines,
            ast_objects: l.ast_objects,
            size_human: l.size_human,
        }
    }
}

/// Converts language stats to MCP total response.
impl From<LanguageStatsOutput> for ShowStatTotalResponse {
    fn from(l: LanguageStatsOutput) -> Self {
        ShowStatTotalResponse {
            files: l.files,
            size_lines: l.size_lines,
            ast_objects: l.ast_objects,
            size_human: l.size_human,
        }
    }
}

/// Converts stats output to MCP response.
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
    /// Returns a formatted tool result with JSON data.
    fn format(&self) -> String {
        format!(
            "[{} language(s), {} file(s), {} line(s)]\nUse this data to respond to the user.\n\n{}",
            self.languages.len(),
            self.total.files,
            self.total.size_lines,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
