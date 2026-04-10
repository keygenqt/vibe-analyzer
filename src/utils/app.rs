//! CLI application helpers for vibe-analyzer.
//! Provides version information, help text formatting, and CLI styling configuration.

use clap::builder::Styles;
use colored::Colorize;

/// Returns the application version from Cargo.toml
pub fn app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Returns the application name as a static string slice.
pub fn app_name() -> &'static str {
    "vibe-analyzer"
}

pub fn app_about() -> String {
    format!(
        r#"

{} - personal knowledge base for developers.

{}"#,
        "Vibe Analyzer".bright_green().bold(),
        "Index, analyze, and search across code, docs, and notes".italic()
    )
}

/// Returns CLI styling configuration
pub fn app_styles() -> Styles {
    Styles::styled()
        .header(clap_cargo::style::HEADER)
        .usage(clap_cargo::style::USAGE)
        .literal(clap_cargo::style::LITERAL)
        .placeholder(clap_cargo::style::PLACEHOLDER)
        .error(clap_cargo::style::ERROR)
        .valid(clap_cargo::style::VALID)
        .invalid(clap_cargo::style::INVALID)
}
