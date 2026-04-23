//! MCP tools integration tests
//!
//! cargo test --test mcp_test -- --ignored --nocapture

mod mcp;

use std::time::Duration;

use tracing::Level;

use crate::mcp::framework::mcp_engine::McpEngine;
use crate::mcp::framework::mcp_environment::McpEnvironment;

/// Number of test runs.
const RUNS: usize = 5;

/// Ollama models to test against.
const MODELS: &[&str] = &[
    "qwen2.5-coder:3b-instruct",
    "qwen2.5-coder:7b-instruct",
    "qwen2.5-coder:14b-instruct",
];

/// MCP tool scenarios to execute.
const SCENARIOS: &[&str] = &[
    "list_projects",
    "search_by_code_classes",
    "search_by_code_functions",
    "search_by_code_imports",
    "search_by_code_variables",
    "search_by_docs",
    "stats",
    "structure",
    "sync_index",
];

#[tokio::test]
#[ignore = "requires MCP server and Ollama running"]
async fn test_all_mcp_tools() -> anyhow::Result<()> {
    tracing::info!("Running tests {} time(s)", RUNS);

    for run in 1..=RUNS {
        println!("╔═══════════════════════════════════════╗");
        println!("║                RUN {}/{}                ║", run, RUNS);
        println!("╚═══════════════════════════════════════╝");

        // Arrange
        let env = McpEnvironment::builder()
            .compose_file("tests/mcp/fixtures/opensearch/docker-compose.yml")
            .opensearch_url("http://localhost:9222")
            .config_dir("tests/mcp/fixtures/config")
            .log_level(Level::DEBUG)
            .request_timeout(Duration::from_secs(60))
            .build()?;

        let test = McpEngine::builder()
            .ollama_host("http://192.168.1.10:11434")
            .models(MODELS)
            .scenarios(
                SCENARIOS
                    .into_iter()
                    .map(|it| format!("{}/{}.json", "tests/mcp/fixtures/scenarios", it)),
            )
            .environment(env)
            .build()?;

        // Act
        let reports = test.run().await?;

        // Assert
        assert!(!reports.is_empty(), "Run {}: No reports generated", run);
    }

    println!("╔═══════════════════════════════════════╗");
    println!("║             ALL RUNS DONE             ║");
    println!("╚═══════════════════════════════════════╝");

    Ok(())
}
