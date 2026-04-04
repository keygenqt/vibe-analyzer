use super::parser::AstData;
use anyhow::Result;

/// Trait for language-specific AST parsers
pub trait LanguageParser: Send + Sync {
    /// Parses source code content and returns AST data
    fn parse(&self, content: &str) -> Result<AstData>;
}
