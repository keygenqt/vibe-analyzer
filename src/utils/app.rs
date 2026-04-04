use clap::builder::Styles;
use colored::Colorize;

/// Returns the application version from Cargo.toml
pub fn app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Returns the application help description
pub fn app_about() -> String {
    format!(
        r#"

{} - fast code indexer with AST analysis for OpenSearch MCP.

{}"#,
        "Vibe Analyzer".bright_green().bold(),
        "Tool for code analysis and search across codebase".italic()
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
