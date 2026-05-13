//! Review enrichment prompt.
//! Surface-level review: bugs, spelling, refactoring suggestions.

use crate::models::file_language::FileLanguageModel;
use crate::ollama::client::OllamaClient;

#[allow(unused_variables)]
impl OllamaClient {
    /// Build prompt for review enrichment.
    pub fn enrich_debt_prompt(files: &[FileLanguageModel]) -> String {
        String::new()
    }

    /// Parse Review enrichment response into model.
    pub fn enrich_debt_parse(
        original: &FileLanguageModel,
        item: &serde_json::Value,
    ) -> FileLanguageModel {
        original.clone()
    }
}
