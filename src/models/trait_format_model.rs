//! Format model trait.
//! Common formatting interface for all models in the pipeline.

/// Common formatting interface for all models.
pub trait TraitFormatModel {
    /// Structure only export, without LLM enrichment.
    fn format_ast(&self) -> Self;

    /// Structure with LLM enrichment export.
    fn format_analyze(&self) -> Self;

    /// Indexing export: structure + enrichment, no content/hash.
    fn format_index(&self) -> Self;

    /// Tool export: minimal representation for MCP responses.
    fn format_tool(&self) -> Self;
}
