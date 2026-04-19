//! MCP test configuration and runner.
//! Holds validated parameters and executes test scenarios.

use anyhow::Result;
use serde_json::Value;
use serde_json::json;

use crate::mcp::framework::mcp_client::McpClient;
use crate::mcp::framework::mcp_environment::McpEnvironment;
use crate::mcp::framework::mcp_report::McpReport;
use crate::mcp::framework::mcp_report::QueryReport;
use crate::mcp::framework::mcp_report::ToolCall;
use crate::mcp::framework::mcp_scenario::McpScenario;
use crate::mcp::framework::mcp_test_builder::McpEngineBuilder;
use crate::mcp::framework::ollama_client::OllamaClient;
use crate::mcp::framework::ollama_client::OllamaResponse;

/// Complete MCP test configuration with execution logic.
#[derive(Debug)]
pub struct McpEngine {
    pub ollama_client: OllamaClient,
    pub models: Vec<String>,
    pub scenarios: Vec<McpScenario>,
    pub environment: McpEnvironment,
}

impl McpEngine {
    /// Creates a new builder for constructing the test configuration.
    pub fn builder() -> McpEngineBuilder {
        McpEngineBuilder::new()
    }

    /// Runs all test scenarios against all models and returns a report.
    pub async fn run(mut self) -> Result<Vec<McpReport>> {
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

        let mut reports = Vec::new();

        for model in &self.models {
            for scenario in &self.scenarios {
                let mut queries = Vec::new();

                for query in &scenario.queries {
                    tracing::info!("Start query ({}): '{}'", model, query);

                    self.environment.log(
                        "LLM request",
                        &[
                            ("model", model),
                            ("query", query),
                            ("tools", &tool_names_str),
                        ],
                    );

                    let request_time = chrono::Utc::now();
                    let response = self.ollama_client.chat(model, query, &tool_values).await?;
                    let (tool_name, tool_args, tool_result, final_response) = self
                        .process_response(&mcp_client, model, query, response)
                        .await?;
                    let duration_ms = (chrono::Utc::now() - request_time).num_milliseconds() as u64;

                    self.environment.log(
                        "LLM final response",
                        &[("model", model), ("response", &final_response)],
                    );

                    queries.push(QueryReport {
                        query: query.clone(),
                        tool_calls: vec![ToolCall {
                            name: tool_name.unwrap_or_else(|| scenario.tool.clone()),
                            args: tool_args.unwrap_or_else(|| "{}".to_string()),
                            result: tool_result.unwrap_or_default(),
                        }],
                        response: final_response,
                        duration_ms,
                        timestamp: request_time.to_rfc3339(),
                    });
                }

                reports.push(McpReport::new(
                    scenario.tool.clone(),
                    model.clone(),
                    queries,
                ));
            }
        }

        Ok(reports)
    }

    /// Processes LLM response and handles tool calls.
    /// Returns (tool_name, tool_args, tool_result, final_response)
    async fn process_response(
        &self,
        mcp_client: &McpClient,
        model: &str,
        query: &str,
        response: OllamaResponse,
    ) -> Result<(Option<String>, Option<String>, Option<String>, String)> {
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
            // Support both "name" (standard) and "function" (legacy/hallucination)
            let name = parsed
                .get("name")
                .or_else(|| parsed.get("function"))
                .or_else(|| parsed.get("tool"))
                .or_else(|| parsed.get("method"))
                .or_else(|| parsed.get("call"))
                .and_then(|n| n.as_str());

            if let Some(name) = name {
                let args = parsed.get("arguments").cloned().unwrap_or(json!({}));
                let args_str = format!("{}", args);

                let tool_name = Some(name.to_string());
                let tool_args = Some(args_str.clone());

                self.environment.log(
                    "LLM tool call detected",
                    &[("model", model), ("tool", name), ("args", &args_str)],
                );

                // Call tool — let it fail if something wrong
                let result = mcp_client.call_tool(name.to_string(), args).await?;

                // If result is empty — that's a bug, fail the test
                if result.is_null() {
                    anyhow::bail!("Tool '{}' returned null for query: '{}'", name, query);
                }

                let result_str = serde_json::to_string_pretty(&result)?;
                let tool_result = Some(result_str.clone());

                let follow_up = self
                .ollama_client
                .chat_simple(
                    model,
                    &format!(
                        "You successfully called tool '{}'. Result:\n{}\n\nProvide a concise natural language response to: '{}'",
                        name, result_str, query
                    ),
                )
                .await?;

                return Ok((tool_name, tool_args, tool_result, follow_up.message.content));
            }
        }

        // Model didn't call a tool when it should — that's a bug
        anyhow::bail!(
            "Model did not call any tool for query: '{}'. Response: '{}'",
            query,
            content
        );
    }
}
