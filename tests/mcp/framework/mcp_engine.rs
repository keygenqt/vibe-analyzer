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
use crate::mcp::framework::ollama_client::ChatMessage;
use crate::mcp::framework::ollama_client::OllamaClient;

/// Max turns per query to prevent infinite loops.
const MAX_TURNS: usize = 5;

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

                    let request_time = chrono::Utc::now();

                    // Run dialog with model
                    let (tool_calls, final_response) = self
                        .run_dialog(&mcp_client, model, query, &tool_values, &tool_names_str)
                        .await?;

                    let duration_ms = (chrono::Utc::now() - request_time).num_milliseconds() as u64;

                    queries.push(QueryReport {
                        query: query.clone(),
                        tool_calls,
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

    /// Runs a multi-turn dialog for a single query.
    async fn run_dialog(
        &self,
        mcp_client: &McpClient,
        model: &str,
        query: &str,
        tool_values: &[Value],
        tool_names_str: &str,
    ) -> Result<(Vec<ToolCall>, String)> {
        let session_id = uuid::Uuid::new_v4().to_string();
        // Fresh history for each query
        let mut messages = vec![ChatMessage::user(query)];
        let mut tool_calls = Vec::new();
        let final_response;

        self.environment.log(
            "LLM dialog start",
            &[
                ("model", model),
                ("query", query),
                ("tools", tool_names_str),
            ],
        );

        // Main dialog loop - model decides when to stop
        loop {
            // Safety check - prevent infinite loops
            if tool_calls.len() >= MAX_TURNS {
                tracing::warn!(
                    "Reached MAX_TURNS ({}), breaking loop to prevent infinite dialog",
                    MAX_TURNS
                );
                final_response = "Max turns reached without final response".to_string();
                break;
            }

            // TOOL AVAILABILITY STRATEGY FOR OLLAMA MCP
            //
            // Problem: Ollama API does not support `finish_reason` field like OpenAI.
            // Models will infinitely generate tool calls if tools are provided on every turn.
            // Tested on 3B, 7B, 14B models — all fail with infinite loops when tools are always available.
            //
            // Solution: Provide tools ONLY on the first turn. After the first tool call,
            // force the model to respond with text by passing an empty tools array.
            // This ensures exactly one tool call per user query and prevents infinite loops.
            //
            // Result: 100% stability across all tested models and scenarios.
            // Trade-off: Multi-tool chains are not supported. Each tool must be self-contained.
            //
            let current_tools = if tool_calls.is_empty() {
                // First turn: model can choose any available tool
                tool_values
            } else {
                // Subsequent turns: model must respond with text only
                &[]
            };

            let response = self
                .ollama_client
                .chat_with_history(&session_id, model, &messages, current_tools)
                .await?;
            let content = response.message.content.trim();

            self.environment.log(
                "LLM raw response",
                &[("model", model), ("raw_content", content)],
            );

            // Parse tool call
            let json_str = self.extract_json(content);

            if let Some((name, args)) = self.parse_tool_call(&json_str).await {
                // Model wants to call a tool
                let args_str = format!("{}", args);

                self.environment.log(
                    "LLM tool call detected",
                    &[("model", model), ("tool", &name), ("args", &args_str)],
                );

                // Call tool
                let result = match mcp_client.call_tool(name.clone(), args.clone()).await {
                    Ok(r) => r,
                    Err(e) => {
                        tracing::warn!("Tool '{}' failed: {}", &name, e);
                        self.environment.log(
                            "LLM tool call failed",
                            &[("model", model), ("tool", &name), ("error", &e.to_string())],
                        );
                        let error_json = json!({ "error": e.to_string() });
                        messages.push(ChatMessage::assistant(content));
                        messages.push(ChatMessage::tool(serde_json::to_string(&error_json)?));
                        continue;
                    }
                };

                if result.is_null() {
                    tracing::error!("Tool '{}' returned null", &name);
                    anyhow::bail!("Tool '{}' returned null for query: '{}'", name, query);
                }

                let result_str = serde_json::to_string_pretty(&result)?;

                tool_calls.push(ToolCall {
                    name: name.clone(),
                    args: args_str,
                    result: result_str.clone(),
                });

                // Add tool call and result to history
                messages.push(ChatMessage::assistant(content));
                messages.push(ChatMessage::tool(result_str));

                continue;
            }

            final_response = response.message.content;
            messages.push(ChatMessage::assistant(final_response.clone()));
            break;
        }

        self.environment.log(
            "LLM dialog end",
            &[
                ("model", model),
                ("turns", &tool_calls.len().to_string()),
                ("response", &final_response),
            ],
        );

        Ok((tool_calls, final_response))
    }

    /// Extracts JSON from markdown code blocks.
    fn extract_json(&self, content: &str) -> String {
        let extracted = if content.starts_with("```json") {
            content
                .strip_prefix("```json")
                .unwrap_or(content)
                .strip_suffix("```")
                .unwrap_or(content)
                .trim()
                .to_string()
        } else if content.starts_with("```") {
            content
                .strip_prefix("```")
                .unwrap_or(content)
                .strip_suffix("```")
                .unwrap_or(content)
                .trim()
                .to_string()
        } else {
            content.to_string()
        };
        extracted
    }

    /// Parses tool call from JSON string.
    async fn parse_tool_call(&self, json_str: &str) -> Option<(String, Value)> {
        let parsed = match serde_json::from_str::<Value>(json_str) {
            Ok(p) => p,
            Err(_) => {
                return None;
            }
        };

        let name = parsed
            .get("name")
            .or_else(|| parsed.get("function"))
            .or_else(|| parsed.get("tool"))
            .or_else(|| parsed.get("method"))
            .or_else(|| parsed.get("call"))
            .and_then(|n| n.as_str())?;

        let args = parsed.get("arguments").cloned().unwrap_or(json!({}));

        Some((name.to_string(), args))
    }
}
