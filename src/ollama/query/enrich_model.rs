//! Ollama API client for LLM-powered code analysis.
//! Handles model management, prompt generation, and response parsing.

use std::time::Duration;

use anyhow::Result;
use colored::Colorize;
use vibe_ast::Language;

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

        // AST mode requires no enrichment — return files unchanged
        if mode == AnalyzeMode::Ast {
            return Ok(enriched);
        }

        // All enrichment modes except Meta require exactly 1 file per batch
        if mode != AnalyzeMode::Meta && files.len() != 1 {
            anyhow::bail!(
                "{} enrichment expects exactly 1 file, got {}",
                mode.name(),
                files.len()
            );
        }

        // Skip Debt for Markdown — not applicable
        if mode == AnalyzeMode::Debt && enriched[0].node_language == Some(Language::Markdown) {
            return Ok(enriched);
        }

        // Build prompt based on enrichment mode
        let prompt = match mode {
            AnalyzeMode::Meta => Self::enrich_meta_prompt(&enriched)?,
            AnalyzeMode::Debt => Self::enrich_debt_prompt(&enriched[0])?,
            AnalyzeMode::Advice => Self::enrich_advice_prompt(&enriched[0])?,
            AnalyzeMode::Errors => Self::enrich_errors_prompt(&enriched[0])?,
            _ => unreachable!(),
        };

        // Select parser for LLM response based on enrichment mode
        let parse_fn = match mode {
            AnalyzeMode::Meta => Self::enrich_meta_parse,
            AnalyzeMode::Debt => Self::enrich_debt_parse,
            AnalyzeMode::Advice => Self::enrich_advice_parse,
            AnalyzeMode::Errors => Self::enrich_errors_parse,
            _ => unreachable!(),
        };

        // Execute prompt and parse response
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

            // @todo
            // if files[0].base_model().path().contains("zizikosh_blogger") {
            //     println!("\n--------------------------");
            //     println!("{}", response);
            //     println!("==========================");
            // }

            match serde_json::from_str::<serde_json::Value>(&response) {
                Ok(json) => {
                    // Single file: response is the DEBT array directly
                    if files.len() == 1 {
                        let model = parse(&files[0], &json);
                        return Ok(vec![model]);
                    }

                    // Multiple files: response is array of objects with PATH
                    let items = json
                        .as_array()
                        .ok_or_else(|| anyhow::anyhow!("Response missing array"))?;

                    let mut enriched: Vec<FileLanguageModel> = Vec::new();
                    for item in items {
                        let path = item["PATH"].as_str().unwrap_or("").to_string();
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
                        items.iter().filter_map(|i| i["PATH"].as_str()).collect();
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
