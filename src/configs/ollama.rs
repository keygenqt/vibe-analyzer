//! Ollama LLM configuration for local model inference.
//! Defines connection parameters and model settings for code analysis.

use serde::Deserialize;
use serde::Serialize;

/// Configuration for Ollama LLM connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    /// Ollama API endpoint URL.
    pub host: String,
    /// Model name to use for inference.
    pub model: String,
    /// Request timeout in seconds.
    pub timeout_secs: u64,
    /// Temperature for generation (0.0-2.0, lower = more deterministic).
    pub temperature: f32,
    /// Seed for reproducible outputs.
    pub seed: u64,
    /// Context window size in tokens.
    pub num_ctx: usize,
    /// Maximum tokens to predict/generate.
    pub num_predict: usize,
}

impl Default for OllamaConfig {
    /// Creates default Ollama configuration.
    fn default() -> Self {
        Self {
            host: "http://localhost:11434".to_string(),
            model: "qwen2.5-coder:3b-instruct".to_string(),
            timeout_secs: 60,
            temperature: 0.1,
            seed: 42,
            num_ctx: 4096,
            num_predict: 2048,
        }
    }
}
