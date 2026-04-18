//! MCP integration tests.
//! Tests MCP tools with Ollama models.

use crate::mcp::framework::{mcp_engine::McpEngine, mcp_environment::McpEnvironment};

#[tokio::test]
#[ignore = "requires MCP server and Ollama running"]
async fn test_single_query() -> anyhow::Result<()> {
    let env = McpEnvironment::new(
        "tests/mcp/fixtures/opensearch/docker-compose.yml",
        "http://localhost:9222",
        "tests/mcp/fixtures/config",
    );

    let test = McpEngine::builder()
        .ollama_host("http://localhost:11434")
        .models(vec!["qwen2.5-coder:3b-instruct"])
        .scenario_file("tests/mcp/fixtures/scenarios/list_projects.json")
        .environment(env)
        .build()?;

    let results = test.run().await?;

    for result in results {
        println!("Model: {}", result.model);
        println!("Response: {}\n", result.response);
    }

    Ok(())
}
