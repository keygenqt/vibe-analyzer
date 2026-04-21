//! HTTP request logging middleware.
//! Logs request details and errors for monitoring and debugging.

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

/// Logs HTTP requests and responses for monitoring and debugging.
pub async fn log_app_responses(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = std::time::Instant::now();

    if method == axum::http::Method::POST {
        let (parts, body) = request.into_parts();
        match axum::body::to_bytes(body, usize::MAX).await {
            Ok(bytes) => {
                let body_str = String::from_utf8_lossy(&bytes);

                // Parse JSON to extract MCP method and tool name
                let (mcp_method, tool_name) = serde_json::from_str::<serde_json::Value>(&body_str)
                    .ok()
                    .map(|json| {
                        let method = json
                            .get("method")
                            .and_then(|m| m.as_str())
                            .map(String::from);
                        let tool = json
                            .get("params")
                            .and_then(|p| p.get("name"))
                            .and_then(|n| n.as_str())
                            .map(String::from);
                        (method, tool)
                    })
                    .unwrap_or((None, None));

                // Prepare body preview for trace logs
                let body_preview = if body_str.len() > 500 {
                    format!("{}... (truncated)", &body_str[..500])
                } else {
                    body_str.to_string()
                };

                // INFO only for important MCP methods
                if let Some(ref m) = mcp_method {
                    if m == "initialize" || m == "tools/call" {
                        if let Some(ref tool) = tool_name {
                            tracing::info!(%method, %uri, mcp.method = %m, tool = %tool, "MCP request");
                        } else {
                            tracing::info!(%method, %uri, mcp.method = %m, "MCP request");
                        }
                        tracing::trace!(%method, %uri, mcp.method = %m, body = %body_preview, "MCP request body");
                    } else {
                        tracing::trace!(%method, %uri, mcp.method = %m, body = %body_preview, "MCP request");
                    }
                } else {
                    tracing::trace!(%method, %uri, body = %body_preview, "POST request");
                }

                let new_request = Request::from_parts(parts, axum::body::Body::from(bytes));
                let response = next.run(new_request).await;
                let status = response.status();
                let elapsed = start.elapsed();

                if status.is_client_error() || status.is_server_error() {
                    tracing::error!(%method, %uri, %status, ?elapsed, "POST request failed");
                } else {
                    tracing::trace!(%method, %uri, %status, ?elapsed, "POST request completed");
                }
                return response;
            }
            Err(e) => {
                tracing::error!(%method, %uri, error = %e, "Failed to read POST body");
                let new_request = Request::from_parts(parts, axum::body::Body::empty());
                return next.run(new_request).await;
            }
        }
    }

    let response = next.run(request).await;
    let status = response.status();
    let elapsed = start.elapsed();

    if status.is_client_error() || status.is_server_error() {
        tracing::error!(%method, %uri, %status, ?elapsed, "HTTP error");
    } else {
        tracing::trace!(%method, %uri, %status, ?elapsed, "HTTP request completed");
    }

    response
}
