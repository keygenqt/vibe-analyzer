//! Review enrichment prompt.
//! Surface-level review: bugs, spelling, refactoring suggestions.

use crate::models::file_language::FileLanguageModel;
use crate::models::trait_base_model::TraitBaseModel;
use crate::models::trait_file_model::TraitFileModel;
use crate::ollama::client::OllamaClient;

// @todo
impl OllamaClient {
    /// Build prompt for technical debt detection.
    pub fn enrich_debt_prompt(files: &[FileLanguageModel]) -> String {
        let template_files: Vec<String> = files
            .iter()
            .map(|f| {
                format!(
                    r#"    {{
      "path": "{}",
      "debt": []
    }}"#,
                    f.base_model().path()
                )
            })
            .collect();

        let template = format!(r#"{{"files": [\n{}\n]}}"#, template_files.join(",\n"));

        format!(
            r#"COPY this JSON EXACTLY, replacing only the placeholders.

Find TODO, FIXME, HACK, @todo in code or comments. Also magic numbers, commented-out code.
Empty [] if none found — do NOT invent or guess.

Also consider: missing error handling, deprecated API usage, hardcoded credentials, duplicate code blocks.

Placeholders to replace:
- "debt" array → list ONLY issues you can SEE in the provided code, or empty []

STRICT RULES:
- Keep ALL {} objects in the array
- Keep ALL "path" values unchanged
- Do NOT skip any object
- Do NOT change JSON structure
- Do NOT report issues you cannot see in the code

JSON TO COPY AND FILL:
{}"#,
            files.len(),
            template
        )
    }

    /// Parse technical debt response into model.
    pub fn enrich_debt_parse(
        original: &FileLanguageModel,
        item: &serde_json::Value,
    ) -> FileLanguageModel {
        let mut model = original.clone();
        model.analyze.debt = item["debt"]
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
