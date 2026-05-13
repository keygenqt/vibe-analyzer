//! Ollama API client for LLM-powered code analysis.
//! Handles model management, prompt generation, and response parsing.

use std::time::Duration;

use anyhow::Result;
use colored::Colorize;

use crate::analyzer::AnalyzeMode;
use crate::models::file_language::FileLanguageModel;
use crate::models::trait_base_model::TraitBaseModel;
use crate::models::trait_file_model::TraitFileModel;
use crate::ollama::client::OllamaClient;

/// Maximum retry attempts for failed enrichment requests.
const MAX_RETRIES: usize = 3;

impl OllamaClient {
    /// Enrich files with LLM based on analysis level.
    pub async fn enrich_model(
        &self,
        mode: AnalyzeMode,
        files: &[FileLanguageModel],
    ) -> Result<Vec<FileLanguageModel>> {
        let enriched = files.to_vec();
        let prompt = match mode {
            AnalyzeMode::Meta => Self::enrich_meta_prompt(&enriched),
            AnalyzeMode::Debt => Self::enrich_debt_prompt(&enriched),
            AnalyzeMode::Advice => Self::enrich_advice_prompt(&enriched),
            AnalyzeMode::Errors => Self::enrich_errors_prompt(&enriched),
            AnalyzeMode::Ast => String::new(),
        };
        let parse_fn = match mode {
            AnalyzeMode::Meta => Self::enrich_meta_parse,
            AnalyzeMode::Debt => Self::enrich_debt_parse,
            AnalyzeMode::Advice => Self::enrich_advice_parse,
            AnalyzeMode::Errors => Self::enrich_errors_parse,
            AnalyzeMode::Ast => return Ok(files.to_vec()),
        };
        self.execute(&prompt, &enriched, parse_fn).await
    }

    /// Execute prompt with retry logic.
    async fn execute(
        &self,
        prompt: &str,
        files: &[FileLanguageModel],
        parse: fn(&FileLanguageModel, &serde_json::Value) -> FileLanguageModel,
    ) -> Result<Vec<FileLanguageModel>> {
        for attempt in 1..=MAX_RETRIES {
            let response = self.call(&prompt, None).await?;
            match serde_json::from_str::<serde_json::Value>(&response) {
                Ok(json) => {
                    let items = json["files"]
                        .as_array()
                        .ok_or_else(|| anyhow::anyhow!("Response missing 'files' array"))?;
                    let mut enriched: Vec<FileLanguageModel> = Vec::new();
                    for item in items {
                        let path = item["path"].as_str().unwrap_or("").to_string();
                        if let Some(original) = files.iter().find(|f| f.base_model().path() == path)
                        {
                            let model = parse(original, item);
                            enriched.push(model);
                        }
                    }
                    if enriched.len() == files.len() {
                        return Ok(enriched);
                    }
                    if attempt < MAX_RETRIES {
                        tokio::time::sleep(Duration::from_millis(500 * attempt as u64)).await;
                        continue;
                    }
                    let sent: Vec<&str> = files.iter().map(|f| f.base_model().path()).collect();
                    let returned: Vec<&str> =
                        items.iter().filter_map(|i| i["path"].as_str()).collect();
                    anyhow::bail!(
                        "LLM returned {} files, expected {}.\n\nExpected paths: {:?}\nReturned paths: {:?}\n\n{}",
                        enriched.len(),
                        files.len(),
                        sent,
                        returned,
                        "Try reducing batch size (analyze.max_chunk_chars) or fix errors in path."
                            .italic()
                    );
                }
                Err(_) if attempt < MAX_RETRIES => {
                    tokio::time::sleep(Duration::from_millis(500 * attempt as u64)).await;
                }
                Err(e) => {
                    anyhow::bail!(
                        "LLM JSON parse failed after {} attempts: {}. Try reducing batch size (analyze.max_chunk_chars).",
                        MAX_RETRIES,
                        e,
                    );
                }
            }
        }
        unreachable!()
    }
}
