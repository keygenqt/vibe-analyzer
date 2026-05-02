//! Ollama API client for LLM-powered code analysis.
//! Handles model management, prompt generation, and response parsing.

use std::time::Duration;

use crate::configs::ollama::OllamaConfig;
use anyhow::Result;
use reqwest::Client;

/// Client for interacting with Ollama API.
#[derive(Clone)]
pub struct OllamaClient {
    pub client: Client,
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

    /// Sends a completion request to Ollama API.
    pub async fn call(&self, prompt: &str, timeout_secs: Option<u64>) -> Result<String> {
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
        .timeout(Duration::from_secs(timeout_secs.unwrap_or(self.config.timeout_secs)))
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                tracing::debug!(
                    prompt_preview = %&prompt[..prompt.len().min(100)],
                    model = %self.config.model,
                    "Ollama request timed out"
                );
                anyhow::anyhow!(
                    "Ollama request timed out ({}s). Try reducing max_chunk_chars in config (current: {})",
                    self.config.timeout_secs,
                    self.config.max_chunk_chars
                )
            } else {
                tracing::debug!(
                    prompt_preview = %&prompt[..prompt.len().min(100)],
                    error = %e,
                    model = %self.config.model,
                    "Ollama request failed"
                );
                anyhow::anyhow!("Ollama request failed: {}", e)
            }
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            tracing::debug!(
                status = %status,
                error = %error_text,
                model = %self.config.model,
                "Ollama API error"
            );
            anyhow::bail!("Ollama API error ({}): {}", status, error_text);
        }

        let json: serde_json::Value = response.json().await?;
        let raw = json["response"].as_str().unwrap_or("");
        Ok(self.response_normalize(raw))
    }

    /// Normalize LLM response: extract JSON from markdown, fix malformed JSON, or return text as-is.
    pub fn response_normalize(&self, raw: &str) -> String {
        let mut content = raw.trim().to_string();

        // Extract content from markdown code block
        if let Some(start_md) = content.find("```") {
            let after_start = &content[start_md + 3..];
            // Skip optional language identifier
            let body_start = after_start.find('\n').map(|i| i + 1).unwrap_or(0);
            let body = &after_start[body_start..];
            if let Some(end_md) = body.find("```") {
                content = body[..end_md].trim().to_string();
            } else {
                content = body.trim().to_string();
            }
        }

        // Not JSON — return as-is
        if !content.contains('{') && !content.contains('[') {
            return content;
        }

        // Find JSON boundaries
        let start = content.find(|c: char| c == '{' || c == '[').unwrap_or(0);
        let end = content
            .rfind(|c: char| c == '}' || c == ']')
            .map(|i| i + 1)
            .unwrap_or(content.len());
        let mut json = content[start..end].trim().to_string();

        // Fix unquoted keys: `key:` → `"key":`
        let key_re = regex::Regex::new(r"(?m)^(\s*)([a-zA-Z_][a-zA-Z0-9_]*)(\s*:)").unwrap();
        json = key_re.replace_all(&json, r#"$1"$2"$3"#).to_string();

        // Fix comma-before-closing: `,}` or `,]`
        let comma_re = regex::Regex::new(r",(\s*[\]}])").unwrap();
        json = comma_re.replace_all(&json, r#"$1"#).to_string();

        // Fix single-quoted strings: `'key'` → `"key"`
        let sq_re = regex::Regex::new(r"'([^']*)'").unwrap();
        json = sq_re.replace_all(&json, r#""$1""#).to_string();

        // Balance braces and brackets
        let mut braces: i32 = 0;
        let mut brackets: i32 = 0;
        for c in json.chars() {
            match c {
                '{' => braces += 1,
                '}' => braces = (braces - 1).max(0),
                '[' => brackets += 1,
                ']' => brackets = (brackets - 1).max(0),
                _ => {}
            }
        }
        if braces > 0 {
            json.push_str(&"}".repeat(braces as usize));
        }
        if brackets > 0 {
            json.push_str(&"]".repeat(brackets as usize));
        }

        json
    }
}
