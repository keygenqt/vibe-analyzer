//! Audit enrichment prompt.
//! Deep analysis: security flaws, performance issues, technical debt.

use crate::models::file_language::FileLanguageModel;
use crate::ollama::client::OllamaClient;

#[allow(unused_variables)]
impl OllamaClient {
    /// Build prompt for audit enrichment.
    pub fn enrich_errors_prompt(files: &[FileLanguageModel]) -> String {
        String::new()
    }

    /// Parse Audit enrichment response into model.
    pub fn enrich_errors_parse(
        original: &FileLanguageModel,
        item: &serde_json::Value,
    ) -> FileLanguageModel {
        original.clone()
    }
}
