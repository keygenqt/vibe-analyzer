//! Explore enrichment prompt.
//! Full exploration: logic analysis, design patterns, architecture recommendations.

use crate::models::file_language::FileLanguageModel;
use crate::models::trait_base_model::TraitBaseModel;
use crate::models::trait_file_model::TraitFileModel;
use crate::ollama::client::OllamaClient;

// @todo
impl OllamaClient {
    /// Build prompt for refactoring and test suggestions.
    pub fn enrich_advice_prompt(files: &[FileLanguageModel]) -> String {
        let template_files: Vec<String> = files
            .iter()
            .map(|f| {
                format!(
                    r#"    {{
      "path": "{}",
      "advice": []
    }}"#,
                    f.base_model().path()
                )
            })
            .collect();

        let template = format!(r#"{{"files": [\n{}\n]}}"#, template_files.join(",\n"));

        format!(
            r#"COPY this JSON EXACTLY, replacing only the placeholders.

Suggest refactoring and tests ACTUALLY NEEDED for the code. Empty [] if none — do NOT invent.

Look for: long functions, repeated logic, unclear naming, missing tests for complex logic.

Placeholders to replace:
- "advice" array → list ONLY suggestions you can justify from the code, or empty []

STRICT RULES:
- Keep ALL {} objects in the array
- Keep ALL "path" values unchanged
- Do NOT skip any object
- Do NOT change JSON structure
- Do NOT invent suggestions not based on the code

JSON TO COPY AND FILL:
{}"#,
            files.len(),
            template
        )
    }

    /// Parse advice response into model.
    pub fn enrich_advice_parse(
        original: &FileLanguageModel,
        item: &serde_json::Value,
    ) -> FileLanguageModel {
        let mut model = original.clone();
        model.analyze.advice = item["advice"]
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
