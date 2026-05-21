//! Enrichment trait and default implementation.
//! Generates prompts for LLM enrichment and parses model responses.

use anyhow::Result;
use serde_json::Value;

use crate::analyzer::AnalyzeMode;
use crate::enrich::advice::AdvicePrompt;
use crate::enrich::debt::DebtPrompt;
use crate::enrich::errors::ErrorsPrompt;
use crate::enrich::meta::MetaPrompt;
use crate::models::file_language::FileLanguageModel;

/// Trait for enrichment prompt generation and response parsing.
pub trait EnrichPrompt {
    /// Build a prompt for the given file and analysis mode.
    fn build_prompt(files: &[FileLanguageModel], mode: AnalyzeMode) -> Result<String>;

    /// Parse model response back into enriched file model.
    fn parse_response(
        originals: &[FileLanguageModel],
        response: &Value,
        mode: AnalyzeMode,
    ) -> Result<Vec<FileLanguageModel>>;
}

/// Default implementation covering all enrichment modes.
pub struct DefaultEnrich;

impl EnrichPrompt for DefaultEnrich {
    fn build_prompt(files: &[FileLanguageModel], mode: AnalyzeMode) -> Result<String> {
        match mode {
            AnalyzeMode::Meta => MetaPrompt::build(files),
            AnalyzeMode::Debt => {
                if let Some(file) = files.first() {
                    DebtPrompt::build(file)
                } else {
                    anyhow::bail!("Debt prompt requires at least 1 file")
                }
            }
            AnalyzeMode::Advice => {
                if let Some(file) = files.first() {
                    AdvicePrompt::build(file)
                } else {
                    anyhow::bail!("Advice prompt requires at least 1 file")
                }
            }
            AnalyzeMode::Errors => {
                if let Some(file) = files.first() {
                    ErrorsPrompt::build(file)
                } else {
                    anyhow::bail!("Errors prompt requires at least 1 file")
                }
            }
            AnalyzeMode::Ast => anyhow::bail!("AST mode does not use enrichment prompts"),
        }
    }

    fn parse_response(
        originals: &[FileLanguageModel],
        response: &serde_json::Value,
        mode: AnalyzeMode,
    ) -> Result<Vec<FileLanguageModel>> {
        match mode {
            AnalyzeMode::Meta => Ok(MetaPrompt::parse(originals, response)),
            AnalyzeMode::Debt => {
                if let Some(original) = originals.first() {
                    Ok(vec![DebtPrompt::parse(original, response)])
                } else {
                    Ok(Vec::new())
                }
            }
            AnalyzeMode::Advice => {
                if let Some(original) = originals.first() {
                    Ok(vec![AdvicePrompt::parse(original, response)])
                } else {
                    Ok(Vec::new())
                }
            }
            AnalyzeMode::Errors => {
                if let Some(original) = originals.first() {
                    Ok(vec![ErrorsPrompt::parse(original, response)])
                } else {
                    Ok(Vec::new())
                }
            }
            AnalyzeMode::Ast => Ok(originals.to_vec()),
        }
    }
}

mod advice;
mod debt;
mod errors;
mod meta;
