//! HTTP request logging middleware.
//! Logs request details and errors for monitoring and debugging.

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

/// Logs HTTP errors for monitoring and debugging.
pub async fn log_app_responses(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    if method == axum::http::Method::POST {
        let (parts, body) = request.into_parts();
        match axum::body::to_bytes(body, usize::MAX).await {
            Ok(bytes) => {
                let body_str = String::from_utf8_lossy(&bytes);
                tracing::info!(%method, %uri, body = %body_str, "POST request");
                let new_request = Request::from_parts(parts, axum::body::Body::from(bytes));
                let response = next.run(new_request).await;
                let status = response.status();
                if status.is_client_error() || status.is_server_error() {
                    tracing::error!(%method, %uri, %status, "POST request failed");
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
    if status.is_client_error() || status.is_server_error() {
        tracing::error!(%method, %uri, %status, "HTTP error");
    }
    response
}
