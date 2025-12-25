//! MCP (Model Context Protocol) Server Implementation for Serena
//!
//! This module provides a pure Rust MCP server that exposes Serena's
//! semantic code analysis tools to AI agents like Claude.

pub mod handler;
pub mod server;
pub mod tools;
pub mod error;

pub use handler::SerenaServerHandler;
pub use server::{start_server, server_details};
pub use error::{SerenaError, SerenaResult};
