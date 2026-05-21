//! Technical debt enrichment prompt and response parsing.
//! Detects code duplication, magic numbers, complex logic, and unsafe calls.

use anyhow::Result;
use serde_json::Value;

use crate::models::file_language::FileLanguageModel;

pub struct DebtPrompt;

impl DebtPrompt {
    /// Build prompt for technical debt detection.
    pub fn build(file: &FileLanguageModel) -> Result<String> {
        let data = serde_json::to_string(file).unwrap_or_default();

        Ok(format!(
            r#"Return a JSON ARRAY OF STRINGS with technical debt issues found. Example: ["issue 1", "issue 2"]

Analyze this code and list technical debt:
- Confusing or overly complex logic
- Code duplication (same logic repeated in multiple places)
- Magic numbers (unexplained numeric literals in logic, not named constants, not 0/1/-1)
- Hardcoded strings used in business logic (not error messages or log texts)
- Function is too long or does too many things
- Unsafe calls: unwrap, expect, panic without error context

RULES:
- Language: English only.
- Report ONLY real problems you see in the code
- Do NOT report what you didn't find ("None found", "No issues")
- Return [] if code is clean

FILE CONTENTS:
{}
"#,
            data,
        ))
    }

    /// Parse technical debt response into model.
    pub fn parse(original: &FileLanguageModel, item: &Value) -> FileLanguageModel {
        let mut model = original.clone();
        model.analyze.debt = item
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
