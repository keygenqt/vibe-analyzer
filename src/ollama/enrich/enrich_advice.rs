//! Explore enrichment prompt.
//! Full exploration: logic analysis, design patterns, architecture recommendations.

use crate::models::file_language::FileLanguageModel;
use crate::ollama::client::OllamaClient;

#[allow(unused_variables)]
impl OllamaClient {
    /// Build prompt for explore enrichment.
    pub fn enrich_advice_prompt(files: &[FileLanguageModel]) -> String {
        String::new()
    }

    /// Parse Explore enrichment response into model.
    pub fn enrich_advice_parse(
        original: &FileLanguageModel,
        item: &serde_json::Value,
    ) -> FileLanguageModel {
        original.clone()
    }
}
