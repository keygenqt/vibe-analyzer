//! Language parser trait definition.
//! Defines the common interface that all language-specific AST parsers must implement.

use anyhow::Result;

use crate::scanner::ast::data::ast_data::AstData;

/// Trait for language-specific AST parsers
pub trait LanguageParser: Send + Sync {
    /// Parses source code content and returns AST data
    fn parse(&self, content: &str) -> Result<AstData>;
}
