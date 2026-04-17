//! MCP test configuration and runner.
//! Holds validated parameters and executes test scenarios.

use anyhow::Result;
use serde_json::{Value, json};

use crate::mcp::framework::{
    mcp_client::McpClient,
    mcp_environment::McpEnvironment,
    mcp_scenario::McpScenario,
    mcp_test_builder::McpEngineBuilder,
    ollama_client::{OllamaClient, OllamaResponse},
    ollama_result::OllamaResult,
};

/// Complete MCP test configuration with execution logic.
#[derive(Debug)]
pub struct McpEngine {
    pub ollama_client: OllamaClient,
    pub models: Vec<String>,
    pub scenario: McpScenario,
    pub environment: McpEnvironment,
}

impl McpEngine {
    /// Creates a new builder for constructing the test configuration.
    pub fn builder() -> McpEngineBuilder {
        McpEngineBuilder::new()
    }

    /// Runs the test scenario against all models and returns results.
    pub async fn run(mut self) -> Result<Vec<OllamaResult>> {
        self.environment.start().await?;
        self.environment.index_fixtures()?;

        let mcp_server_url = self.environment.mcp_url();
        let mcp_client = McpClient::new(&mcp_server_url).await?;
        let tools = mcp_client.list_tools().await?;

        let tool_values: Vec<serde_json::Value> = tools
            .iter()
            .map(|t| {
                json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.input_schema,
                    }
                })
            })
            .collect();

        let mut results = Vec::new();

        for model in &self.models {
            for query in &self.scenario.queries {
                let response = self.ollama_client.chat(model, query, &tool_values).await?;
                let final_response = self
                    .process_response(&mcp_client, model, query, response)
                    .await?;

                results.push(OllamaResult {
                    tool: self.scenario.tool.clone(),
                    model: model.clone(),
                    response: final_response,
                });
            }
        }

        Ok(results)
    }

    async fn process_response(
        &self,
        mcp_client: &McpClient,
        model: &str,
        query: &str,
        response: OllamaResponse,
    ) -> Result<String> {
        let content = response.message.content.trim();

        let json_str = if content.starts_with("```json") {
            content
                .strip_prefix("```json")
                .unwrap_or(content)
                .strip_suffix("```")
                .unwrap_or(content)
                .trim()
        } else if content.starts_with("```") {
            content
                .strip_prefix("```")
                .unwrap_or(content)
                .strip_suffix("```")
                .unwrap_or(content)
                .trim()
        } else {
            content
        };

        if let Ok(parsed) = serde_json::from_str::<Value>(json_str) {
            if let Some(tool_name) = parsed.get("name").and_then(|n| n.as_str()) {
                let tool_args = parsed.get("arguments").cloned().unwrap_or(json!({}));

                println!(
                    "[DEBUG] Calling tool: {} with args: {}",
                    tool_name, tool_args
                );

                let tool_result = mcp_client
                    .call_tool(tool_name.to_string(), tool_args)
                    .await?;
                let tool_result_str = serde_json::to_string_pretty(&tool_result)?;

                let follow_up = self
                    .ollama_client
                    .chat_simple(
                        model,
                        &format!(
                            "You called the tool '{}' and got this result:\n{}\n\nNow provide a natural language response to the original query: '{}'",
                            tool_name, tool_result_str, query
                        ),
                    )
                    .await?;

                return Ok(follow_up.message.content);
            }
        }

        Ok(response.message.content)
    }
}
