//! MCP test configuration and runner.
//! Holds validated parameters and executes test scenarios.

use anyhow::Result;
use serde_json::{Value, json};

use crate::mcp::framework::mcp_client::McpClient;
use crate::mcp::framework::mcp_environment::McpEnvironment;
use crate::mcp::framework::mcp_report::{McpReport, QueryReport, ToolCall};
use crate::mcp::framework::mcp_scenario::McpScenario;
use crate::mcp::framework::mcp_test_builder::McpEngineBuilder;
use crate::mcp::framework::ollama_client::{OllamaClient, OllamaResponse};

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

    /// Runs the test scenario against all models and returns a report.
    pub async fn run(mut self) -> Result<McpReport> {
        self.environment.start().await?;
        self.environment.index_fixtures()?;

        let mcp_server_url = self.environment.mcp_url();
        let mcp_client = McpClient::new(&mcp_server_url).await?;
        let tools = mcp_client.list_tools().await?;

        let tool_names: Vec<String> = tools.iter().map(|t| t.name.to_string()).collect();
        let tool_names_str = format!("{:?}", tool_names);

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

        let model = self.models.first().cloned().unwrap_or_default();
        let mut queries = Vec::new();

        for query in &self.scenario.queries {
            tracing::info!("Start query ({}): '{}'", model, query);

            self.environment.log(
                "LLM request",
                &[
                    ("model", &model),
                    ("query", query),
                    ("tools", &tool_names_str),
                ],
            );

            let request_time = chrono::Utc::now();
            let response = self.ollama_client.chat(&model, query, &tool_values).await?;
            let final_response = self
                .process_response(&mcp_client, &model, query, response)
                .await?;
            let duration_ms = (chrono::Utc::now() - request_time).num_milliseconds() as u64;

            self.environment.log(
                "LLM final response",
                &[("model", &model), ("response", &final_response)],
            );

            queries.push(QueryReport {
                query: query.clone(),
                tool_calls: vec![ToolCall {
                    name: self.scenario.tool.clone(),
                    args: "{}".to_string(),
                }],
                response: final_response,
                duration_ms,
                timestamp: request_time.to_rfc3339(),
            });
        }

        Ok(McpReport::new(self.scenario.tool.clone(), model, queries))
    }

    async fn process_response(
        &self,
        mcp_client: &McpClient,
        model: &str,
        query: &str,
        response: OllamaResponse,
    ) -> Result<String> {
        let content = response.message.content.trim();

        self.environment.log(
            "LLM raw response",
            &[("model", model), ("raw_content", content)],
        );

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
                let tool_args_str = format!("{}", tool_args);

                self.environment.log(
                    "LLM tool call detected",
                    &[
                        ("model", model),
                        ("tool", tool_name),
                        ("args", &tool_args_str),
                    ],
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
