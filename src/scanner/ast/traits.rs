//! Language parser trait definition
//!
//! This module defines the [`LanguageParser`] trait that all language-specific
//! AST parsers must implement.

use super::parser::AstData;
use anyhow::Result;

/// Trait for language-specific AST parsers
pub trait LanguageParser: Send + Sync {
    /// Parses source code content and returns AST data
    ///
    /// # Arguments
    /// * `content` - Source code as a UTF-8 string
    ///
    /// # Returns
    /// * `Ok(AstData)` - Extracted AST information (may be empty)
    /// * `Err` - If parsing fails due to invalid syntax or grammar errors
    fn parse(&self, content: &str) -> Result<AstData>;
}
