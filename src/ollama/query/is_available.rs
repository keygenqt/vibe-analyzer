//! Ollama API client for LLM-powered code analysis.
//! Handles model management, prompt generation, and response parsing.

use std::time::Duration;

use crate::ollama::client::OllamaClient;
use anyhow::Result;

impl OllamaClient {
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
}
