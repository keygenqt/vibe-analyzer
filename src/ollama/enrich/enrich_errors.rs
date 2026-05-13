//! Audit enrichment prompt.
//! Deep analysis: security flaws, performance issues, technical debt.

use crate::models::file_language::FileLanguageModel;
use crate::models::trait_base_model::TraitBaseModel;
use crate::models::trait_file_model::TraitFileModel;
use crate::ollama::client::OllamaClient;

// @todo
impl OllamaClient {
    /// Build prompt for bug and spelling error detection.
    pub fn enrich_errors_prompt(files: &[FileLanguageModel]) -> String {
        let template_files: Vec<String> = files
            .iter()
            .map(|f| {
                format!(
                    r#"    {{
      "path": "{}",
      "errors": []
    }}"#,
                    f.base_model().path()
                )
            })
            .collect();

        let template = format!(r#"{{"files": [\n{}\n]}}"#, template_files.join(",\n"));

        format!(
            r#"COPY this JSON EXACTLY, replacing only the placeholders.

Find bugs and spelling errors ACTUALLY PRESENT in the code. Empty [] if none found — do NOT invent or guess.

Look for: null safety issues, division by zero, incorrect error handling, typos in comments and strings.

Placeholders to replace:
- "errors" array → list ONLY issues you can SEE in the provided code, or empty []

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

    /// Parse errors response into model.
    pub fn enrich_errors_parse(
        original: &FileLanguageModel,
        item: &serde_json::Value,
    ) -> FileLanguageModel {
        let mut model = original.clone();
        model.analyze.errors = item["errors"]
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
