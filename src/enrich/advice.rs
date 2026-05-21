//! Advice enrichment prompt and response parsing.
//! Suggests refactoring, test improvements, and documentation enhancements.

use anyhow::Result;
use serde_json::Value;
use vibe_ast::Language;

use crate::models::file_language::FileLanguageModel;

pub struct AdvicePrompt;

impl AdvicePrompt {
    /// Build prompt for refactoring and test suggestions.
    pub fn build(file: &FileLanguageModel) -> Result<String> {
        let data = serde_json::to_string(file).unwrap_or_default();
        if file.node_language == Some(Language::Markdown) {
            Ok(Self::markdown_prompt(data))
        } else {
            Ok(Self::code_prompt(data))
        }
    }

    /// Parse advice response into model.
    pub fn parse(original: &FileLanguageModel, item: &Value) -> FileLanguageModel {
        let mut model = original.clone();
        model.analyze.advice = item
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        model
    }

    /// Build prompt for refactoring and test suggestions in code.
    fn code_prompt(data: String) -> String {
        format!(
            r#"Return a JSON ARRAY OF STRINGS with refactoring and test suggestions. Example: ["suggestion 1", "suggestion 2"]

Analyze this code and suggest:
- How to simplify complex functions
- Missing tests for critical logic
- Better naming for unclear variables or functions
- Opportunities to reduce code duplication

RULES:
- Language: English only.
- Suggest ONLY improvements you can justify from the code
- Do NOT invent problems
- Return [] if code is well-written

FILE CONTENTS:
{}
"#,
            data,
        )
    }

    /// Build prompt for documentation improvement suggestions in markdown.
    fn markdown_prompt(data: String) -> String {
        format!(
            r#"Return a JSON ARRAY OF STRINGS with suggestions to improve this documentation. Example: ["suggestion 1", "suggestion 2"]

Analyze this markdown and suggest:
- Missing sections or information
- Structure improvements (heading levels, ordering)
- Clarity improvements

RULES:
- Language: English only.
- Suggest ONLY real improvements
- Return [] if documentation is good

FILE CONTENTS:
{}
"#,
            data,
        )
    }
}
