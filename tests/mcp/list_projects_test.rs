//! MCP integration tests.
//! Tests MCP tools with Ollama models.

use std::path::PathBuf;

use crate::mcp::framework::mcp_engine::McpEngine;
use crate::mcp::framework::mcp_environment::McpEnvironment;

#[tokio::test]
#[ignore = "requires MCP server and Ollama running"]
async fn test_list_projects() -> anyhow::Result<()> {
    // Arrange
    let env = McpEnvironment::new(
        "tests/mcp/fixtures/opensearch/docker-compose.yml",
        "http://localhost:9222",
        "tests/mcp/fixtures/config",
    );
    let test = McpEngine::builder()
        .ollama_host("http://192.168.1.10:11434")
        .models(vec!["qwen2.5-coder:3b-instruct"])
        .scenario_file("tests/mcp/fixtures/scenarios/list_projects.json")
        .environment(env)
        .build()?;

    // Act
    let report = test.run().await?;
    let report_dir = PathBuf::from("tests/mcp/reports");
    report.save(&report_dir)?;
    report.print_summary();

    // Assert
    assert!(report_dir.exists());
    Ok(())
}
