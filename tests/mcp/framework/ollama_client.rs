//! Ollama API client.
//! Provides chat completion with tool calling support.

use std::time::Duration;

use anyhow::Result;
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;

/// Chat message structure.
#[derive(Debug, Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }

    pub fn tool(content: impl Into<String>) -> Self {
        Self {
            role: "tool".to_string(),
            content: content.into(),
        }
    }
}

/// Client for interacting with Ollama API.
#[derive(Debug)]
pub struct OllamaClient {
    client: Client,
    host: String,
    timeout: Duration,
}

impl OllamaClient {
    /// Creates a new Ollama client with the given host URL.
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            host: host.into(),
            timeout: Duration::from_secs(60),
        }
    }

    /// Sets the request timeout.
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// Sends a chat request with message history and tools.
    pub async fn chat_with_history(
        &self,
        id: &str,
        model: &str,
        messages: &[ChatMessage],
        tools: &[Value],
    ) -> Result<OllamaResponse> {
        let url = format!("{}/api/chat", self.host);
        let body = json!({
            "model": model,
            "messages": messages,
            "tools": tools,
            "stream": false,
            "session_id": id.to_string(),
            "options": {
                "temperature": 0.1,
                "seed": 42,
                "num_ctx": 4096,
                "num_predict": 1024,
            }
        });

        let response = self
            .client
            .post(&url)
            .timeout(self.timeout)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    anyhow::anyhow!("Ollama request timed out after {:?}", self.timeout)
                } else {
                    anyhow::anyhow!("Ollama request failed: {}", e)
                }
            })?
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
