//! Ollama API client for LLM-powered code analysis.
//! Handles model management, prompt generation, and response parsing.

use std::time::Duration;

use anyhow::Result;

use crate::{models::file_language::FileLanguageModel, ollama::client::OllamaClient};

/// Maximum retry attempts for failed enrichment requests.
const MAX_RETRIES: usize = 3;

impl OllamaClient {
    /// Enrich a batch of files with LLM-generated descriptions and tags.
    pub async fn enrich_model(
        &self,
        files: &[FileLanguageModel],
    ) -> Result<Vec<FileLanguageModel>> {
        let prompt = Self::build_enrich_prompt(files);
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
                        if let Some(original) = files.iter().find(|f| f.path == path) {
                            let mut model = original.clone();
                            model.summary = item["description"].as_str().map(|s| s.to_string());
                            let llm_tags: Vec<String> = item["tags"]
                                .as_array()
                                .map(|a| {
                                    a.iter()
                                        .filter_map(|v| v.as_str().map(String::from))
                                        .collect()
                                })
                                .unwrap_or_default();
                            model.tags.extend(llm_tags);
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
                    anyhow::bail!(
                        "LLM returned {} files, expected {}. Reduce max_chunk_chars ({}) or max_chunk_files ({})",
                        enriched.len(),
                        files.len(),
                        self.config.max_chunk_chars,
                        self.config.max_chunk_files,
                    );
                }
                Err(_) if attempt < MAX_RETRIES => {
                    tokio::time::sleep(Duration::from_millis(500 * attempt as u64)).await;
                }
                Err(e) => {
                    anyhow::bail!(
                        "LLM JSON parse failed after {} attempts: {}. Reduce max_chunk_chars ({})",
                        MAX_RETRIES,
                        e,
                        self.config.max_chunk_chars,
                    );
                }
            }
        }
        unreachable!()
    }

    /// Build enrichment prompt with JSON template for weak models.
    fn build_enrich_prompt(files: &[FileLanguageModel]) -> String {
        let template_files: Vec<String> = files
            .iter()
            .map(|f| {
                format!(
                    r#"    {{
      "path": "{}",
      "description": "FILL_DESCRIPTION",
      "tags": ["TAG1", "TAG2", "TAG3", "TAG4", "TAG5"]
    }}"#,
                    f.path
                )
            })
            .collect();

        let template = format!(r#"{{"files": [\n{}\n]}}"#, template_files.join(",\n"));

        format!(
            r#"COPY this JSON EXACTLY, replacing only the placeholders.

Placeholders to replace:
- "FILL_DESCRIPTION" → write 2-3 sentences about the file
- "TAG1", "TAG2", "TAG3", "TAG4", "TAG5" → replace with 3-5 specific keywords

STRICT RULES:
- Keep ALL {} objects in the array
- Keep ALL "path" values unchanged
- Do NOT skip any object
- Do NOT change JSON structure

JSON TO COPY AND FILL:
{}"#,
            files.len(),
            template
        )
    }
}
