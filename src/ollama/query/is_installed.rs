//! Ollama API client for LLM-powered code analysis.
//! Handles model management, prompt generation, and response parsing.

use crate::ollama::client::OllamaClient;
use anyhow::Result;

impl OllamaClient {
    /// Check if the configured model is installed.
    pub async fn is_installed(&self) -> Result<bool> {
        let url = format!("{}/api/tags", self.config.host);
        let json: serde_json::Value = self.client.get(&url).send().await?.json().await?;
        Ok(json["models"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .any(|m| m["name"].as_str() == Some(&self.config.model)))
    }
}
