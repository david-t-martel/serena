//! Web transport layer for MCP (Model Context Protocol)
//!
//! This crate provides HTTP and Server-Sent Events (SSE) transport implementations
//! for the Model Context Protocol, enabling web-based communication with MCP servers.

pub mod http;
pub mod server;
pub mod sse;

pub use http::HttpTransport;
pub use server::WebServer;
pub use sse::SseTransport;

/// Re-export commonly used types from serena-mcp
pub use serena_mcp::{McpError, McpRequest, McpResponse, SerenaMcpServer};
