//! Ollama API client for LLM-powered code analysis.
//! Handles model management, prompt generation, and response parsing.

use anyhow::Result;

use crate::ollama::client::OllamaClient;

impl OllamaClient {
    /// Generate project summary from README content.
    pub async fn summary(&self, readme: &str) -> Result<String> {
        let prompt = r#"
Extract or write a concise 2-3 sentence summary of what this project does.
Focus on the project's purpose and main functionality.
Return only the summary text.
---
"#;
        self.call(
            &format!("{}{}", prompt, &readme[..readme.len().min(3000)]),
            None,
        )
        .await
        .map(|r| r.trim().to_string())
    }
}
