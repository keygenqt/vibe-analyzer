//! Ollama API client for LLM-powered code analysis.
//! Handles model management, prompt generation, and response parsing.

use std::time::Duration;

use crate::analyzer::data::output::enriched_file_analysis::EnrichedFileAnalysis;
use crate::analyzer::data::requests::ollama_files_request::OllamaFilesRequest;
use crate::analyzer::data::requests::ollama_summary_request::OllamaSummaryRequest;
use crate::analyzer::data::responses::ollama_enrichment_response::OllamaEnrichmentResponse;
use crate::analyzer::data::responses::ollama_generate_response::OllamaGenerateResponse;
use crate::analyzer::data::responses::ollama_tags_response::OllamaTagsResponse;
use crate::configs::ollama::OllamaConfig;
use anyhow::Result;
use reqwest::Client;

/// Client for interacting with Ollama API.
#[derive(Clone)]
pub struct OllamaClient {
    client: Client,
    pub config: OllamaConfig,
}

impl OllamaClient {
    /// Creates a new Ollama client from configuration.
    pub fn new(config: &OllamaConfig) -> Self {
        Self {
            client: Client::new(),
            config: config.clone(),
        }
    }

    /// Checks if Ollama service is available.
    pub async fn is_available(&self) -> Result<bool> {
        match self
            .client
            .get(&self.config.host)
            .timeout(Duration::from_secs(10))
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Checks if the configured model is already installed.
    pub async fn is_model_installed(&self) -> Result<bool> {
        let url = format!("{}/api/tags", self.config.host);
        let response = self.client.get(&url).send().await?;
        let tags: OllamaTagsResponse = response.json().await?;

        Ok(tags.models.iter().any(|m| m.name == self.config.model))
    }

    /// Checks if Ollama service is available and warms up the model.
    pub async fn ensure_ready(&self) -> Result<()> {
        // Check if Ollama server is reachable
        match self.client.get(&self.config.host).send().await {
            Ok(_) => {}
            Err(e) => {
                anyhow::bail!("Ollama not available at {}: {}", self.config.host, e);
            }
        }

        // Check if model is installed
        if !self.is_model_installed().await? {
            anyhow::bail!(
                "Model '{}' not found. Install it with: ollama pull {}",
                self.config.model,
                self.config.model
            );
        }

        // Warm up model (load into memory)
        let url = format!("{}/api/generate", self.config.host);
        let request = serde_json::json!({
            "model": self.config.model,
            "prompt": "",
            "stream": false,
        });

        self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to warm up model: {}", e))?;

        Ok(())
    }

    /// Generates source summary using LLM.
    pub async fn get_summary_enriched_analysis(
        &self,
        data: &OllamaSummaryRequest,
    ) -> Result<String> {
        let json_data = serde_json::to_string(data)?;

        let prompt = format!(
            r#"Analyze this source and provide a concise 2-3 sentence summary in English.

Source data:
{}

Provide only the summary text, no additional formatting."#,
            json_data
        );

        let response = self.generate_completion(&prompt).await?;
        Ok(response.trim().to_string())
    }

    /// Enriches a batch of files with LLM-generated descriptions and search tags.
    pub async fn get_file_enriched_analysis(
        &self,
        data: &OllamaFilesRequest,
    ) -> Result<Vec<EnrichedFileAnalysis>> {
        const MAX_RETRIES: usize = 5;
        let template_files: Vec<String> = data
            .files
            .iter()
            .map(|f| {
                format!(
                    r#"    {{
      "path": "{}",
      "description": "FILL_DESCRIPTION",
      "tags": ["TAG1", "TAG2", "TAG3"]
    }}"#,
                    f.path
                )
            })
            .collect();

        let template = format!(
            r#"{{"files": [
{}
]}}"#,
            template_files.join(",\n")
        );

        let prompt = format!(
            r#"COPY this JSON EXACTLY, replacing only the placeholders.

Placeholders to replace:
- "FILL_DESCRIPTION" → write 2-3 sentences about the file
- "TAG1", "TAG2", "TAG3" → replace with 3-5 specific keywords

STRICT RULES:
- Keep ALL {} objects in the array
- Keep ALL "path" values unchanged
- Do NOT skip any object
- Do NOT change JSON structure

JSON TO COPY AND FILL:
{}"#,
            data.files.len(),
            template
        );

        for attempt in 1..=MAX_RETRIES {
            let raw_response = self.generate_completion(&prompt).await?;
            let json_str = self.clean_llm_json(&raw_response);

            match serde_json::from_str::<OllamaEnrichmentResponse>(&json_str) {
                Ok(parsed) => {
                    let mut enriched = Vec::new();
                    for file_response in &parsed.files {
                        if let Some(original) =
                            data.files.iter().find(|f| f.path == file_response.path)
                        {
                            enriched.push(EnrichedFileAnalysis {
                                description: Some(file_response.description.clone()),
                                tags: Some(file_response.tags.clone()),
                                path: original.path.clone(),
                                extension: original.extension.clone(),
                                is_text: original.is_text,
                                size_bytes: original.size_bytes,
                                lines_of_code: original.lines_of_code,
                                size_chars: original.size_chars,
                                ast: original.ast.clone(),
                                hash: original.hash.clone(),
                            });
                        }
                    }

                    if enriched.len() == data.files.len() {
                        return Ok(enriched);
                    }

                    if attempt < MAX_RETRIES {
                        tokio::time::sleep(Duration::from_millis(500 * attempt as u64)).await;
                        continue;
                    }

                    anyhow::bail!(
                        "LLM returned {} files, expected {}. Response may be truncated or paths mismatch.\n\
                            \n\
                            Troubleshooting:\n\
                            - Try reducing max_chunk_chars ({}) or max_chunk_files ({}) in config\n\
                            - Try a more capable model (e.g., qwen2.5-coder:7b, llama3.2:3b)\n\
                            - Split large files into smaller batches\n\
                            \n\
                            Config location: ~/.vibe-analyzer/config.json",
                        enriched.len(),
                        data.files.len(),
                        self.config.max_chunk_chars,
                        self.config.max_chunk_files
                    );
                }
                Err(_) if attempt < MAX_RETRIES => {
                    tokio::time::sleep(Duration::from_millis(500 * attempt as u64)).await;
                }
                Err(e) => {
                    let preview = &json_str[..json_str.len().min(1000)];
                    anyhow::bail!(
                        "LLM JSON parse failed after {} attempts: {}...\nReduce max_chunk_chars (current: {})\n{}\n...",
                        MAX_RETRIES,
                        e,
                        self.config.max_chunk_chars,
                        preview,
                    );
                }
            }
        }
        unreachable!()
    }

    /// Sends a completion request to Ollama API.
    async fn generate_completion(&self, prompt: &str) -> Result<String> {
        let url = format!("{}/api/generate", self.config.host);

        let request = serde_json::json!({
            "model": self.config.model,
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": self.config.temperature,
                "seed": self.config.seed,
                "num_ctx": self.config.num_ctx,
                "num_predict": self.config.num_predict,
            }
        });

        let response = self.client
        .post(&url)
        .json(&request)
        .timeout(Duration::from_secs(self.config.timeout_secs))
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                anyhow::anyhow!(
                    "Ollama request timed out ({}s). Try reducing max_chunk_chars in config (current: {})",
                    self.config.timeout_secs,
                    self.config.max_chunk_chars
                )
            } else {
                anyhow::anyhow!("Ollama request failed: {}", e)
            }
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            anyhow::bail!("Ollama API error ({}): {}", status, error_text);
        }

        let generate_response: OllamaGenerateResponse = response.json().await?;
        Ok(generate_response.response)
    }

    /// Cleans and repairs malformed JSON from LLM responses.
    fn clean_llm_json(&self, raw: &str) -> String {
        let mut content = raw.trim().to_string();

        // Extract JSON from markdown code blocks
        if let Some(start_md) = content.find("```") {
            let after_start = &content[start_md + 3..];
            let json_start = after_start.find('\n').map(|i| i + 1).unwrap_or(0);
            let remainder = &after_start[json_start..];

            if let Some(end_md) = remainder.find("```") {
                content = remainder[..end_md].to_string();
            } else {
                content = remainder.to_string();
            }
        }

        // Locate JSON object or array boundaries
        let start = content.find(|c| c == '{' || c == '[').unwrap_or(0);
        let end = content
            .rfind(|c| c == '}' || c == ']')
            .map(|i| i + 1)
            .unwrap_or(content.len());
        let mut json_str = content[start..end].trim().to_string();

        // Fix unquoted object keys: , key: -> , "key":
        let key_regex = regex::Regex::new(r"(?m)([{,]\s*)([a-zA-Z_][a-zA-Z0-9_]*)(\s*:)").unwrap();
        json_str = key_regex.replace_all(&json_str, r#"$1"$2"$3"#).to_string();

        // Fix trailing commas: [1, 2, ] -> [1, 2]
        let trailing_comma_regex = regex::Regex::new(r",(\s*[\]}])").unwrap();
        json_str = trailing_comma_regex
            .replace_all(&json_str, r#"$1"#)
            .to_string();

        // Balance unmatched braces and brackets
        let mut open_braces = 0;
        let mut open_brackets = 0;

        for c in json_str.chars() {
            match c {
                '{' => open_braces += 1,
                '}' => {
                    if open_braces > 0 {
                        open_braces -= 1
                    }
                }
                '[' => open_brackets += 1,
                ']' => {
                    if open_brackets > 0 {
                        open_brackets -= 1
                    }
                }
                _ => {}
            }
        }

        json_str.push_str(&"}".repeat(open_braces));
        json_str.push_str(&"]".repeat(open_brackets));

        json_str
    }
}
