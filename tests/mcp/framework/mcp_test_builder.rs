//! Builder for MCP test configuration.
//! Provides step-by-step construction with validation.

use anyhow::Result;
use std::fs;

use crate::mcp::framework::mcp_engine::McpEngine;
use crate::mcp::framework::mcp_environment::McpEnvironment;
use crate::mcp::framework::mcp_scenario::McpScenario;
use crate::mcp::framework::ollama_client::OllamaClient;

/// Builder for constructing a validated MCP test configuration.
#[derive(Debug, Default)]
pub struct McpEngineBuilder {
    ollama_host: Option<String>,
    models: Option<Vec<String>>,
    scenario_file: Option<String>,
    environment: Option<McpEnvironment>,
}

impl McpEngineBuilder {
    /// Creates a new builder with default empty values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the Ollama API host URL.
    pub fn ollama_host(mut self, host: impl Into<String>) -> Self {
        self.ollama_host = Some(host.into());
        self
    }

    /// Sets the list of models to test.
    pub fn models(mut self, models: Vec<&str>) -> Self {
        self.models = Some(models.into_iter().map(String::from).collect());
        self
    }

    /// Sets the path to the scenario JSON file.
    pub fn scenario_file(mut self, path: impl Into<String>) -> Self {
        self.scenario_file = Some(path.into());
        self
    }

    /// Sets the test environment.
    pub fn environment(mut self, env: McpEnvironment) -> Self {
        self.environment = Some(env);
        self
    }

    /// Validates, loads scenario, and builds the MCP test configuration.
    pub fn build(self) -> Result<McpEngine> {
        let ollama_host = self
            .ollama_host
            .ok_or_else(|| anyhow::anyhow!("Ollama host not specified"))?;
        let models = self
            .models
            .ok_or_else(|| anyhow::anyhow!("Models not specified"))?;
        let scenario_file = self
            .scenario_file
            .ok_or_else(|| anyhow::anyhow!("Scenario file not specified"))?;
        let environment = self
            .environment
            .ok_or_else(|| anyhow::anyhow!("Test environment not specified"))?;

        let content = fs::read_to_string(&scenario_file)?;
        let scenario: McpScenario = serde_json::from_str(&content)?;
        let ollama_client = OllamaClient::new(ollama_host);

        Ok(McpEngine {
            ollama_client,
            models,
            scenario,
            environment,
        })
    }
}
