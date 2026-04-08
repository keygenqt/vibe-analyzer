//! Ollama API client for LLM-powered code analysis.
//! Handles model management, prompt generation, and response parsing.

use std::time::Duration;

use crate::{
    analyzer::data::{
        output::enriched_file_analysis::EnrichedFileAnalysis,
        requests::{
            ollama_file_request::OllamaFilesRequest, ollama_summary_request::OllamaSummaryRequest,
        },
        responses::{
            ollama_enrichment_response::OllamaEnrichmentResponse,
            ollama_generate_response::OllamaGenerateResponse,
            ollama_tags_response::OllamaTagsResponse,
        },
    },
    configs::ollama::OllamaConfig,
};
use anyhow::Result;
use reqwest::Client;

/// Client for interacting with Ollama API.
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
        match self.client.get(&self.config.host).send().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Checks if the configured model is already installed.
    async fn is_model_installed(&self) -> Result<bool> {
        let url = format!("{}/api/tags", self.config.host);
        let response = self.client.get(&url).send().await?;
        let tags: OllamaTagsResponse = response.json().await?;

        Ok(tags.models.iter().any(|m| m.name == self.config.model))
    }

    /// Checks if Ollama service is available and warms up the model.
    pub async fn ensure_ready(&self) -> Result<()> {
        // 1. Check if Ollama server is reachable
        match self.client.get(&self.config.host).send().await {
            Ok(_) => {}
            Err(e) => {
                anyhow::bail!("Ollama not available at {}: {}", self.config.host, e);
            }
        }

        // 2. Check if model is installed
        if !self.is_model_installed().await? {
            anyhow::bail!(
                "Model '{}' not found. Install it with: ollama pull {}",
                self.config.model,
                self.config.model
            );
        }

        // 3. Warm up model (load into memory)
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

    /// Generates project summary using LLM.
    pub async fn get_summary_enriched_analysis(
        &self,
        data: &OllamaSummaryRequest,
    ) -> Result<String> {
        let json_data = serde_json::to_string(data)?;

        let prompt = format!(
            r#"Analyze this project and provide a concise 2-3 sentence summary in English.

Project data:
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
        let files_json = format!("[{}]", data.values.join(","));

        let prompt = format!(
            r#"Analyze these code files. Return ONLY valid JSON:
{{
  "files": [
    {{
      "file_id": "...",
      "description": "2-3 sentences describing the file's purpose and role",
      "tags": ["tag1", "tag2", "tag3", ...]
    }}
  ]
}}

Rules:
- Use EXACT file_id from input
- Provide 3-5 specific tags per file (technologies, patterns, concepts)
- Descriptions in English only

Files:
{}"#,
            files_json
        );

        let response = self.generate_completion(&prompt).await?;

        // Extract JSON from response (handle potential markdown code blocks)
        let json_str = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        let parsed: OllamaEnrichmentResponse = serde_json::from_str(json_str).map_err(|e| {
            let preview = if json_str.len() > 400 {
                format!("{}...", &json_str[..400])
            } else {
                json_str.to_string()
            };

            anyhow::anyhow!(
                "LLM returned invalid JSON: {}\n\
             Response preview: {}\n\
             Try reducing max_chunk_chars in config (current: {})",
                e,
                preview,
                self.config.max_chunk_chars
            )
        })?;

        // Convert response to EnrichedFileAnalysis
        let mut enriched = Vec::new();
        for file_response in parsed.files {
            // Find matching original file to preserve all fields
            if let Some(original_file) = data
                .files
                .iter()
                .find(|f| f.file_id == file_response.file_id)
            {
                enriched.push(EnrichedFileAnalysis {
                    base: original_file.clone(),
                    description: Some(file_response.description),
                    tags: Some(file_response.tags),
                });
            }
        }

        Ok(enriched)
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
}
