//! MCP HTTP server implementation using Axum and rmcp.
//! Provides JSON-RPC endpoint for AI assistant integration.

use axum::Json;
use axum::Router;
use axum::ServiceExt;
use axum::extract::FromRef;
use axum::extract::Request;
use axum::middleware::from_fn;
use axum::routing::get;
use rmcp::model::ProtocolVersion;
use rmcp::transport::StreamableHttpServerConfig;
use rmcp::transport::streamable_http_server::StreamableHttpService;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use std::path::PathBuf;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::normalize_path::NormalizePathLayer;

use crate::configs::app::AppConfig;
use crate::mcp::log;
use crate::mcp::router::RouterTools;
use crate::opensearch::client::OpenSearchClient;
use crate::opensearch::search::SearchClient;
use crate::utils::app;

/// Application state shared across request handlers.
#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub search_client: Arc<SearchClient>,
    pub protocol_version: ProtocolVersion,
}

/// Start MCP server.
pub async fn run_server(
    host: Option<String>,
    port: Option<u16>,
    workdir: Option<PathBuf>,
    protocol: Option<String>,
) -> anyhow::Result<()> {
    let config = AppConfig::load()?;
    let mcp_config = config.mcp.clone();
    let host_str = host.unwrap_or(mcp_config.host);
    let port_val = port.unwrap_or(mcp_config.port);

    // Get protocol version
    let protocol_version_str = protocol.unwrap_or(mcp_config.protocol);
    let protocol_version = match protocol_version_str.as_str() {
        "2024-11-05" => ProtocolVersion::V_2024_11_05,
        "2025-03-26" => ProtocolVersion::V_2025_03_26,
        "2025-06-18" => ProtocolVersion::V_2025_06_18,
        _ => ProtocolVersion::LATEST,
    };

    if let Some(dir) = workdir {
        std::env::set_current_dir(&dir)?;
    }

    let client = OpenSearchClient::new(&config.opensearch.host)?;
    let state = AppState {
        config: Arc::new(config.clone()),
        search_client: Arc::new(SearchClient::new(client)),
        protocol_version: protocol_version.clone(),
    };

    let config = StreamableHttpServerConfig::default()
        .disable_allowed_hosts()
        .with_stateful_mode(true)
        .with_json_response(false);

    let state_mcp = state.clone();
    let mcp_service = StreamableHttpService::new(
        move || {
            let internal_use = state_mcp.clone();
            let state = Arc::new(internal_use);
            Ok(RouterTools::new(state))
        },
        LocalSessionManager::default().into(),
        config,
    );

    let router = Router::new()
        .route(
            "/",
            get(|| async {
                Json(serde_json::json!({
                    "name": app::app_name(),
                    "version": app::app_version(),
                    "status": "OK"
                }))
            }),
        )
        .route("/mcp", get(|| async { "OK" }))
        .route("/mcp/health", get(|| async { "OK" }))
        .nest_service("/mcp/v1", mcp_service)
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(from_fn(log::log_app_responses));

    let route_str = format!("{}:{}", host_str, port_val);
    let listener = tokio::net::TcpListener::bind(&route_str).await?;

    let app = ServiceBuilder::new()
        .layer(NormalizePathLayer::trim_trailing_slash())
        .service(router);
    let service = ServiceExt::<Request>::into_make_service(app);

    tracing::info!(
        "MCP server listening on http://{} (protocol {})",
        listener.local_addr()?,
        protocol_version.as_str()
    );

    axum::serve(listener, service).await?;
    Ok(())
}
