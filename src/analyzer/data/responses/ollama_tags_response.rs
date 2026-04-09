//! Ollama tags API response structure.
//! Contains list of all installed models available for inference.

use serde::Deserialize;
use serde::Serialize;

use crate::analyzer::data::responses::ollama_model_info_response::OllamaModelInfoResponse;

/// Response from Ollama /api/tags endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaTagsResponse {
    /// List of installed models
    pub models: Vec<OllamaModelInfoResponse>,
}
