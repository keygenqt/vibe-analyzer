//! Explore enrichment prompt.
//! Full exploration: logic analysis, design patterns, architecture recommendations.

use anyhow::Result;

use crate::models::file_language::FileLanguageModel;
use crate::ollama::client::OllamaClient;

impl OllamaClient {
    /// Build prompt for refactoring and test suggestions.
    pub fn enrich_advice_prompt(_: &FileLanguageModel) -> Result<String> {
        anyhow::bail!("Advice enrichment not yet implemented");
    }

    /// Parse advice response into model.
    pub fn enrich_advice_parse(
        original: &FileLanguageModel,
        _: &serde_json::Value,
    ) -> FileLanguageModel {
        original.clone()
    }
}
