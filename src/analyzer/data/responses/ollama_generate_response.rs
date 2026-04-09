//! Ollama generate API response structure.
//! Contains the generated text completion from the model.

use serde::Deserialize;
use serde::Serialize;

/// Response from Ollama /api/generate endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaGenerateResponse {
    /// Generated text completion from the model
    pub response: String,
}
