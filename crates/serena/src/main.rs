mod app;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing::{info, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use app::App;

/// Serena - AI-powered coding assistant with LSP and MCP support
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Transport mode: stdio, http, or sse
    #[arg(short, long, default_value = "stdio")]
    transport: String,

    /// HTTP server port (for http/sse transports)
    #[arg(short, long, default_value = "3000")]
    port: u16,

    /// Project directory to activate
    #[arg(short = 'P', long)]
    project: Option<PathBuf>,

    /// Configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Operating mode (planning, editing, interactive, one-shot)
    #[arg(short, long)]
    mode: Option<String>,

    /// Context to use (desktop-app, agent, ide-assistant)
    #[arg(long)]
    context: Option<String>,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Set up tracing/logging
    setup_tracing(&args)?;

    info!("Starting Serena v{}", env!("CARGO_PKG_VERSION"));
    info!("Transport: {}, Port: {}", args.transport, args.port);

    // Create and initialize application
    let mut app = App::new(args.config, args.project).await?;

    // Apply mode and context if specified
    if let Some(mode) = args.mode {
        app.set_mode(&mode)?;
    }
    if let Some(context) = args.context {
        app.set_context(&context)?;
    }

    // Run the server based on transport type
    match args.transport.as_str() {
        "stdio" => {
            info!("Starting MCP server on stdio transport");
            app.run_stdio().await?;
        }
        "http" => {
            info!("Starting MCP server on HTTP transport (port {})", args.port);
            app.run_http(args.port).await?;
        }
        "sse" => {
            info!("Starting MCP server on SSE transport (port {})", args.port);
            app.run_sse(args.port).await?;
        }
        _ => {
            anyhow::bail!("Invalid transport type: {}", args.transport);
        }
    }

    info!("Serena shutting down gracefully");
    Ok(())
}

/// Set up tracing subscriber with appropriate log level
fn setup_tracing(args: &Args) -> Result<()> {
    let log_level = if args.verbose {
        "debug"
    } else {
        &args.log_level
    };

    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(log_level))
        .unwrap_or_else(|_| {
            warn!("Invalid log level '{}', defaulting to 'info'", log_level);
            EnvFilter::new("info")
        });

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(env_filter)
        .init();

    Ok(())
}
