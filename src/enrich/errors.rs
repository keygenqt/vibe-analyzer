//! Errors enrichment prompt and response parsing.
//! Detects bugs, spelling errors, unsafe calls, and incorrect error handling.

use anyhow::Result;
use serde_json::Value;
use vibe_ast::Language;

use crate::models::file_language::FileLanguageModel;

pub struct ErrorsPrompt;

impl ErrorsPrompt {
    /// Build prompt for bug and spelling error detection.
    pub fn build(file: &FileLanguageModel) -> Result<String> {
        let data = serde_json::to_string(file).unwrap_or_default();
        if file.node_language == Some(Language::Markdown) {
            Ok(Self::markdown_prompt(data))
        } else {
            Ok(Self::code_prompt(data))
        }
    }

    /// Parse errors response into model.
    pub fn parse(original: &FileLanguageModel, item: &Value) -> FileLanguageModel {
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

    /// Build prompt for bug and spelling error detection in code.
    fn code_prompt(data: String) -> String {
        format!(
            r#"Return a JSON ARRAY OF STRINGS with bugs and errors found. Example: ["issue 1", "issue 2"]

Analyze this code and list issues:
- Unsafe calls: unwrap, expect, panic without error context
- Incorrect error handling (swallowed errors, empty catch)

RULES:
- Language: English only.
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
    fn markdown_prompt(data: String) -> String {
        format!(
            r#"You are a text editor. Find spelling and grammar errors in this text. Return a JSON ARRAY OF STRINGS with errors found. Return [] if text is clean.

LANGUAGE: Use the same language as the document text. Find errors in that language.

FILE CONTENTS:
{}
"#,
            data,
        )
    }
}
