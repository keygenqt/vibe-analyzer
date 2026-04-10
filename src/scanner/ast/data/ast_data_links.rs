//! AST data structures for struct/record definitions.
//! Stores struct signatures and associated documentation comments.

use serde::Deserialize;
use serde::Serialize;

/// Markdown links structure with text and URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstLink {
    pub text: String,
    pub url: String,
}
