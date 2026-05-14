//! Audit enrichment prompt.
//! Deep analysis: security flaws, performance issues, technical debt.

use anyhow::Result;
use vibe_ast::Language;

use crate::models::file_language::FileLanguageModel;
use crate::ollama::client::OllamaClient;

impl OllamaClient {
    /// Build prompt for bug and spelling error detection.
    pub fn enrich_errors_prompt(file: &FileLanguageModel) -> Result<String> {
        let data = serde_json::to_string(file).unwrap_or_default();
        if file.node_language == Some(Language::Markdown) {
            Ok(Self::enrich_errors_prompt_markdown(data))
        } else {
            Ok(Self::enrich_errors_prompt_code(data))
        }
    }

    /// Build prompt for bug and spelling error detection in code.
    pub fn enrich_errors_prompt_code(data: String) -> String {
        format!(
            r#"Return a JSON ARRAY OF STRINGS with bugs and errors found. Example: ["issue 1", "issue 2"]

Analyze this code and list issues:
- Unsafe calls: unwrap, expect, panic without error context
- Incorrect error handling (swallowed errors, empty catch)

RULES:
- Report ONLY real problems you see in the code
- Do NOT report what you didn't find ("None found", "No issues")
- Return [] if code is clean

FILE CONTENTS:
{}
"#,
            data,
        )
    }

    /// Build prompt for spelling and grammar error detection in markdown.
    pub fn enrich_errors_prompt_markdown(data: String) -> String {
        format!(
            r#"You are a text editor. Find spelling and grammar errors in this text. Return a JSON ARRAY OF STRINGS with errors found. Return [] if text is clean.

FILE CONTENTS:
{}
"#,
            data,
        )
    }

    /// Parse errors response into model.
    pub fn enrich_errors_parse(
        original: &FileLanguageModel,
        item: &serde_json::Value,
    ) -> FileLanguageModel {
        let mut model = original.clone();
        model.analyze.errors = item
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        model
    }
}
