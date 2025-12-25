//! Serena MCP Server - Binary Entry Point
//!
//! This is the main entry point for the Serena MCP server.
//! It provides a stdio-based MCP server for semantic code operations.

use clap::Parser;
use serena_core::mcp::{start_server, tools::cli::CommandArguments};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    init_logging();

    // Parse command line arguments
    let args = CommandArguments::parse();

    // Start the MCP server
    if let Err(e) = start_server(args).await {
        tracing::error!("Server error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

fn init_logging() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();
}
