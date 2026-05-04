//! AST model trait.
//! Common interface for all AST element types.

/// Common interface for all AST elements.
pub trait AstModel {
    /// AST-only export: structure without enrichment.
    fn format_ast(&self) -> Self;

    /// AST + enrichment export.
    fn format_analyze(&self) -> Self;

    /// Indexing export: structure + enrichment, no content/hash.
    fn format_index(&self) -> Self;

    /// Tool export: minimal representation for MCP responses.
    fn format_tool(&self) -> Self;
}
