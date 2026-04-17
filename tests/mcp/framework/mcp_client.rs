//! MCP client using rmcp streamable HTTP transport.
//! Provides connection to MCP server and tool calling.

use anyhow::Result;
use rmcp::{
    RoleClient, ServiceExt,
    model::{
        CallToolRequestParams, ClientCapabilities, ClientInfo, Implementation,
        InitializeRequestParams, Tool,
    },
    service::RunningService,
    transport::StreamableHttpClientTransport,
};
use serde_json::{Value, json};

/// Client for interacting with MCP server via streamable HTTP.
pub struct McpClient {
    client: RunningService<RoleClient, InitializeRequestParams>,
}

impl McpClient {
    /// Creates a new MCP client connected to the given server URL.
    pub async fn new(server_url: &str) -> Result<Self> {
        let transport = StreamableHttpClientTransport::from_uri(server_url);
        let client_info = ClientInfo::new(
            ClientCapabilities::default(),
            Implementation::new("mcp-test-runner", "0.1.0"),
        );
        let client: RunningService<RoleClient, InitializeRequestParams> =
            client_info.serve(transport).await?;
        Ok(Self { client })
    }

    /// Lists all available tools from the MCP server.
    pub async fn list_tools(&self) -> Result<Vec<Tool>> {
        let response = self.client.list_tools(None).await?;
        Ok(response.tools)
    }

    /// Calls a tool with the given name and arguments.
    pub async fn call_tool(&self, name: String, args: Value) -> Result<Value> {
        let params = if let Some(obj) = args.as_object() {
            CallToolRequestParams::new(name).with_arguments(obj.clone())
        } else {
            CallToolRequestParams::new(name)
        };
        let result = self.client.call_tool(params).await?;

        let content = result
            .content
            .into_iter()
            .next()
            .map(|c| json!({ "raw": format!("{:?}", c) }))
            .unwrap_or(Value::Null);

        Ok(content)
    }
}
