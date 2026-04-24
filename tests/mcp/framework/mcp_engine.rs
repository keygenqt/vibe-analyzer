//! MCP test configuration and runner.
//! Holds validated parameters and executes test scenarios.

use std::time::Duration;

use anyhow::Result;
use serde_json::Value;
use serde_json::json;
use tokio::time::sleep;

use crate::mcp::framework::mcp_client::McpClient;
use crate::mcp::framework::mcp_engine_builder::McpEngineBuilder;
use crate::mcp::framework::mcp_environment::McpEnvironment;
use crate::mcp::framework::mcp_report::McpReport;
use crate::mcp::framework::mcp_report::QueryReport;
use crate::mcp::framework::mcp_report::ToolCall;
use crate::mcp::framework::mcp_scenario::McpScenario;
use crate::mcp::framework::ollama_client::ChatMessage;
use crate::mcp::framework::ollama_client::OllamaClient;

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
        self.environment.index_fixtures().await?;
        self.ollama_client
            .set_timeout(self.environment.request_timeout);

        let reports_dir = self
            .environment
            .reports_dir
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Reports directory not set"))?;

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

                    sleep(Duration::from_secs(1)).await;

                    queries.push(QueryReport {
                        query: query.clone(),
                        tool_calls,
                        response: final_response,
                        duration_ms,
                        timestamp: request_time.to_rfc3339(),
                    });

                    // Save intermediate report after each query
                    let intermediate_report =
                        McpReport::new(scenario.tool.clone(), model.clone(), queries.clone());
                    intermediate_report.save(&reports_dir)?;
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

    /// Runs a single-turn dialog for a query.
    /// Model MUST call exactly one tool, then provide final response.
    async fn run_dialog(
        &self,
        mcp_client: &McpClient,
        model: &str,
        query: &str,
        tool_values: &[Value],
        tool_names_str: &str,
    ) -> Result<(Vec<ToolCall>, String)> {
        let session_id = uuid::Uuid::new_v4().to_string();
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

        // === TURN 1: Model MUST call a tool ===
        let response = self
            .ollama_client
            .chat_with_history(&session_id, model, &messages, tool_values)
            .await?;
        let content = response.message.content.trim();

        self.environment.log(
            "LLM turn 1 response",
            &[("model", model), ("raw_content", content)],
        );

        let json_str = self.extract_json(content);

        if let Some((name, args)) = self.parse_tool_call(&json_str).await {
            let args_str = format!("{}", args);

            self.environment.log(
                "LLM tool call detected",
                &[("model", model), ("tool", &name), ("args", &args_str)],
            );

            // Call tool — server validates and resolves aliases
            let result = match mcp_client
                .call_tool(name.clone(), args.clone(), self.environment.request_timeout)
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    let error_str = e.to_string();
                    if error_str.starts_with("TOOL_NOT_FOUND:") {
                        tracing::error!("Model '{}' called non-existent tool '{}'.", model, name);
                        final_response =
                            format!("[ERROR] Model called non-existent tool: {}", name);
                        return Ok((tool_calls, final_response));
                    }
                    tracing::error!("Tool '{}' failed: {}", &name, e);
                    final_response = format!("[ERROR] Tool '{}' failed: {}", name, e);
                    return Ok((tool_calls, final_response));
                }
            };

            if result.is_null() {
                tracing::error!("Tool '{}' returned null", &name);
                final_response = format!("[ERROR] Tool '{}' returned null", name);
                return Ok((tool_calls, final_response));
            }

            let result_str = serde_json::to_string(&result)?;

            tool_calls.push(ToolCall {
                name: name.clone(),
                args: args_str,
                result: result_str.clone(),
            });

            messages.push(ChatMessage::assistant(content));
            messages.push(ChatMessage::tool(result_str));
        } else {
            tracing::error!(
                "Model '{}' did not call a tool on first turn. Response: {}",
                model,
                content
            );
            final_response = format!("[ERROR] Model did not call a tool: {}", content);
            return Ok((tool_calls, final_response));
        }

        // === TURN 2: Model MUST provide final text response (no tools) ===
        let response = self
            .ollama_client
            .chat_with_history(&session_id, model, &messages, &[])
            .await?;
        let content = response.message.content.trim();

        self.environment.log(
            "LLM turn 2 response",
            &[("model", model), ("raw_content", content)],
        );

        // Check if model tried to call another tool instead of responding
        let json_str = self.extract_json(content);
        if let Some((tool_name, _)) = self.parse_tool_call(&json_str).await {
            tracing::warn!(
                "Model '{}' tried to call another tool '{}' instead of providing final response",
                model,
                tool_name
            );
            final_response = format!(
                "[ERROR] Model called second tool '{}' instead of final response: {}",
                tool_name, content
            );
        } else {
            final_response = content.to_string();
            messages.push(ChatMessage::assistant(final_response.clone()));
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
