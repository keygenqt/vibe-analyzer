//! MCP JSON wrapper for consistent serialization.
//! Provides custom JSON extractor and response for MCP endpoints.

use axum::Json;
use axum::extract::FromRequest;
use axum::response::IntoResponse;
use axum::response::Response;

use crate::mcp::api::mcp_response::McpResponse;

/// Application custom JSON wrapper.
#[derive(FromRequest)]
#[from_request(via(Json), rejection(McpResponse))]
pub struct McpJson<T>(pub T);

/// Impl axum response for McpJson.
impl<T> IntoResponse for McpJson<T>
where
    Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        Json(self.0).into_response()
    }
}
