//! MCP (Model Context Protocol) module for vibe-analyzer.
//! Provides HTTP server implementation for AI assistant integration
//! with JSON-RPC endpoint and tool execution.

pub mod api;
pub mod handlers;
pub mod models;
pub mod route;
pub mod server;
pub mod tools;
