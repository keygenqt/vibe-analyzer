//! File analysis data for OpenSearch indexing.
//! Contains AST, enrichment, and searchable metadata.

use crate::analyzer::data::output::analyzer_file_output::AnalyzerFileOutput;
use crate::analyzer::data::output::analyzer_project_output::AnalyzerProjectOutput;
use crate::opensearch::data::HasPath;
use crate::scanner::ast::data::ast_data::AstData;
use serde::Deserialize;
use serde::Serialize;

/// File analysis stored in vibe_smart_{hash} index.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "FileStats")]
pub struct OpensearchFileStatsRequest {
    /// Absolute path to source root
    pub root: String,
    /// Relative path from source root (used as document _id)
    pub path: String,
    /// File extension without dot (e.g., "rs", "toml")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    /// Whether the file is text-based
    pub is_text: bool,
    /// File size in bytes
    pub size_bytes: u64,
    /// Number of lines (text files only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_of_code: Option<usize>,
    /// Number of characters (text files only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_chars: Option<usize>,
    /// AST data (if text and parsable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ast: Option<AstData>,
    /// LLM-generated description of file purpose
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Search tags for improved discoverability
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Unique file identifier (BLAKE3 hash of content)
    pub hash: String,
}

impl HasPath for OpensearchFileStatsRequest {
    fn path(&self) -> &str {
        &self.path
    }
}

impl OpensearchFileStatsRequest {
    /// Creates OpensearchFileStatsRequest from EnrichedFileAnalysis.
    fn new(root: &str, data: &AnalyzerFileOutput) -> Self {
        Self {
            root: root.to_string(),
            path: data.path.clone(),
            hash: data.hash.clone(),
            extension: data.extension.clone(),
            is_text: data.is_text,
            size_bytes: data.size_bytes,
            lines_of_code: data.lines_of_code,
            size_chars: data.size_chars,
            ast: data.ast.clone(),
            description: data.description.clone(),
            tags: data.tags.clone(),
        }
    }

    /// Converts EnrichedSourceAnalysis to Vec<SourceFileAnalysis>.
    pub fn from_enriched(source: &AnalyzerProjectOutput) -> Vec<Self> {
        source
            .files
            .iter()
            .map(|f| Self::new(&source.root, f))
            .collect()
    }
}
