//! MCP tools integration tests
//!
//! cargo test --test mcp_test -- --ignored --nocapture

mod mcp;

use crate::mcp::framework::mcp_engine::McpEngine;
use crate::mcp::framework::mcp_environment::McpEnvironment;
use std::path::PathBuf;

/// Path to OpenSearch docker-compose file for test environment.
const COMPOSE_FILE: &str = "tests/mcp/fixtures/opensearch/docker-compose.yml";

/// Ollama API endpoint URL.
const OLLAMA_HOST: &str = "http://192.168.1.10:11434";

/// OpenSearch service URL for health checks and indexing.
const OPENSEARCH_URL: &str = "http://localhost:9222";

/// Configuration directory containing config.json5 and SPDX data.
const CONFIG_DIR: &str = "tests/mcp/fixtures/config";

/// Directory with MCP tool test scenarios in JSON format.
const SCENARIOS_DIR: &str = "tests/mcp/fixtures/scenarios";

/// Output directory for test reports.
const REPORTS_DIR: &str = "tests/reports";

/// Ollama models to test against.
const MODELS: &[&str] = &[
    "qwen2.5-coder:3b-instruct",
    "qwen2.5-coder:7b-instruct",
    "qwen2.5-coder:14b-instruct",
];

/// MCP tool scenarios to execute.
const SCENARIOS: &[&str] = &["tool_get_tree", "tool_index_projects", "tool_list_projects"];

#[tokio::test]
#[ignore = "requires MCP server and Ollama running"]
async fn test_all_mcp_tools() -> anyhow::Result<()> {
    // Arrange
    let env = McpEnvironment::new(COMPOSE_FILE, OPENSEARCH_URL, CONFIG_DIR);
    let test = McpEngine::builder()
        .ollama_host(OLLAMA_HOST)
        .models(MODELS)
        .scenarios(
            SCENARIOS
                .into_iter()
                .map(|it| format!("{}/{}.json", SCENARIOS_DIR, it)),
        )
        .environment(env)
        .build()?;

    // Act
    let reports = test.run().await?;
    let report_dir = PathBuf::from(REPORTS_DIR);

    // Assert
    for report in reports {
        report.save(&report_dir)?;
        report.print_summary();
    }
    Ok(())
}
