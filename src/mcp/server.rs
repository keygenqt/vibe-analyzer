//! MCP HTTP server implementation using Axum.
//! Provides JSON-RPC endpoint for AI assistant integration.

use axum::Router;
use axum::ServiceExt;
use axum::extract::FromRef;
use axum::extract::Request;
use axum::middleware::Next;
use axum::middleware::from_fn;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::routing::post;
use std::path::PathBuf;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::normalize_path::NormalizePathLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry;
use tracing_subscriber::util::SubscriberInitExt;

use crate::configs::app::AppConfig;
use crate::mcp::api::mcp_response::McpResponse;
use crate::mcp::route;
use crate::opensearch::client::OpenSearchClient;
use crate::opensearch::search::SearchClient;

/// Application state shared across request handlers.
#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub search_client: Arc<SearchClient>,
}

/// Start MCP server.
/// @todo - Test compatibility with MCP clients (Continue, Cursor, Cody, Windsurf, etc.)
pub async fn run_server(
    host: Option<String>,
    port: Option<u16>,
    workdir: Option<PathBuf>,
) -> anyhow::Result<()> {
    // Enable server mode for macros (use tracing instead of colored output)
    crate::utils::macros::set_server_mode(true);

    let config = AppConfig::load()?;
    let mcp_config = config.mcp.clone();

    // Change working directory if specified
    if let Some(dir) = workdir {
        std::env::set_current_dir(&dir)?;
    }

    // Initialize OpenSearch client
    let client = OpenSearchClient::new(&config.opensearch.host)?;
    let state = AppState {
        config: Arc::new(config.clone()),
        search_client: Arc::new(SearchClient::new(client)),
    };

    // Configure routes
    let router = Router::new()
        .route("/", get(route::mcp_info::get))
        .route("/mcp", get(|| async { "OK" }))
        .route("/mcp", post(route::mcp_handler::post))
        .route("/mcp/v1", get(|| async { "OK" }))
        .route("/mcp/v1", post(route::mcp_handler::post))
        .with_state(state)
        .fallback(handle_404)
        .layer(from_fn(log_app_responses));

    // Bind listener
    let host = host.unwrap_or(mcp_config.host);
    let port = port.unwrap_or(mcp_config.port);
    let route = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&route).await?;

    // Initialize tracing
    let layer = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into());
    registry().with(layer).with(fmt::layer()).init();

    // Build service
    let app = ServiceBuilder::new()
        .layer(NormalizePathLayer::trim_trailing_slash())
        .service(router);
    let service = ServiceExt::<Request>::into_make_service(app);

    // Show info about connect
    tracing::info!(
        "Server started. Listening on http://{}",
        listener.local_addr()?
    );

    // Start service
    axum::serve(listener, service).await?;

    Ok(())
}

/// Handle 404 Not Found.
async fn handle_404() -> impl IntoResponse {
    McpResponse::server_error(None, "Not found")
}

/// Log responses for error tracing.
async fn log_app_responses(request: Request, next: Next) -> Response {
    let response = next.run(request).await;
    if let Some(mcp_response) = response.extensions().get::<Arc<McpResponse>>() {
        if mcp_response.error.is_some() {
            tracing::error!(?mcp_response, "Error in MCP handler");
        }
    }
    response
}
