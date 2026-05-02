//! Ollama API client for LLM-powered code analysis.
//! Handles model management, prompt generation, and response parsing.

use anyhow::Result;

use crate::ollama::client::OllamaClient;

impl OllamaClient {
    /// Check if Ollama is ready: server reachable and model installed.
    pub async fn is_ready(&self) -> Result<()> {
        // Check server reachable
        if !self.is_available().await? {
            anyhow::bail!("Ollama not available at {}", self.config.host);
        }
        // Check model installed
        if !self.is_installed().await? {
            anyhow::bail!(
                "Model '{}' not found. Install it with: ollama pull {}",
                self.config.model,
                self.config.model
            );
        }
        Ok(())
    }
}
