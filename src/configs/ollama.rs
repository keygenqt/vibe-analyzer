//! Ollama LLM configuration for local model inference.
//! Defines connection parameters and model settings for code analysis.

use serde::Deserialize;
use serde::Serialize;

/// Configuration for Ollama LLM connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    /// Ollama API endpoint URL
    pub host: String,

    /// Model name to use for inference
    pub model: String,

    /// Maximum chunk size in characters for LLM context
    pub max_chunk_chars: usize,
}

impl Default for OllamaConfig {
    /// Creates default Ollama configuration
    fn default() -> Self {
        Self {
            host: "http://localhost:11434".to_string(),
            model: "qwen2.5-coder:7b-instruct".to_string(),
            max_chunk_chars: 100_000,
        }
    }
}
