//! Ollama API client.
//! Provides chat completion with tool calling support.

use anyhow::Result;
use reqwest::Client;
use serde_json::{Value, json};

/// Client for interacting with Ollama API.
#[derive(Debug)]
pub struct OllamaClient {
    client: Client,
    host: String,
}

impl OllamaClient {
    /// Creates a new Ollama client with the given host URL.
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            host: host.into(),
        }
    }

    /// Sends a chat request with tools and returns the response.
    pub async fn chat(&self, model: &str, prompt: &str, tools: &[Value]) -> Result<OllamaResponse> {
        let url = format!("{}/api/chat", self.host);
        let body = json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "tools": tools,
            "stream": false,
        });

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?
            .json::<OllamaResponse>()
            .await?;

        Ok(response)
    }

    /// Sends a simple chat request without tools.
    pub async fn chat_simple(&self, model: &str, prompt: &str) -> Result<OllamaResponse> {
        let url = format!("{}/api/chat", self.host);
        let body = json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "stream": false,
        });

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?
            .json::<OllamaResponse>()
            .await?;

        Ok(response)
    }
}

/// Response from Ollama chat API.
#[derive(Debug, serde::Deserialize)]
pub struct OllamaResponse {
    pub message: OllamaMessage,
}

/// Message from Ollama response.
#[derive(Debug, serde::Deserialize)]
pub struct OllamaMessage {
    pub content: String,
}
