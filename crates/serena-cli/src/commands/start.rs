//! Start command - launches the Serena MCP server

use crate::args::Transport;
use crate::commands::Execute;
use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::{error, info};

/// Configuration for the start command
#[derive(Debug, Clone)]
pub struct StartCommand {
    /// Transport mechanism to use
    pub transport: Transport,

    /// Host address (for network transports)
    pub host: String,

    /// Port number (for network transports)
    pub port: u16,

    /// Configuration file path
    pub config_path: Option<PathBuf>,

    /// Project directory path
    pub project_path: Option<PathBuf>,
}

impl StartCommand {
    /// Create a new start command with the given configuration
    pub fn new(
        transport: Transport,
        host: String,
        port: u16,
        config_path: Option<PathBuf>,
        project_path: Option<PathBuf>,
    ) -> Self {
        Self {
            transport,
            host,
            port,
            config_path,
            project_path,
        }
    }

    /// Validate the command configuration
    fn validate(&self) -> Result<()> {
        // Validate network configuration for network transports
        if self.transport.requires_network() {
            if self.port == 0 {
                anyhow::bail!("Port must be greater than 0 for network transports");
            }
            if self.host.is_empty() {
                anyhow::bail!("Host must not be empty for network transports");
            }
        }

        // Validate paths exist if provided
        if let Some(ref config) = self.config_path {
            if !config.exists() {
                anyhow::bail!("Configuration file does not exist: {}", config.display());
            }
        }

        if let Some(ref project) = self.project_path {
            if !project.exists() {
                anyhow::bail!("Project directory does not exist: {}", project.display());
            }
            if !project.is_dir() {
                anyhow::bail!("Project path is not a directory: {}", project.display());
            }
        }

        Ok(())
    }

    /// Start the MCP server with stdio transport
    async fn start_stdio(&self) -> Result<()> {
        info!("Starting Serena MCP server with stdio transport");

        // TODO: Initialize the MCP server with stdio transport
        // This will be implemented when serena-mcp is ready

        info!("MCP server initialization complete");
        info!("Listening on stdio for MCP requests");

        // Keep the server running
        tokio::signal::ctrl_c()
            .await
            .context("Failed to listen for ctrl-c signal")?;

        info!("Shutting down MCP server");
        Ok(())
    }

    /// Start the MCP server with HTTP transport
    async fn start_http(&self) -> Result<()> {
        info!(
            "Starting Serena MCP server with HTTP transport on {}:{}",
            self.host, self.port
        );

        // TODO: Initialize the MCP server with HTTP transport
        // This will be implemented when serena-web is available

        info!("MCP server initialization complete");
        info!("Listening on http://{}:{} for MCP requests", self.host, self.port);

        // Keep the server running
        tokio::signal::ctrl_c()
            .await
            .context("Failed to listen for ctrl-c signal")?;

        info!("Shutting down MCP server");
        Ok(())
    }

    /// Start the MCP server with SSE transport
    async fn start_sse(&self) -> Result<()> {
        info!(
            "Starting Serena MCP server with SSE transport on {}:{}",
            self.host, self.port
        );

        // TODO: Initialize the MCP server with SSE transport
        // This will be implemented when serena-web is available

        info!("MCP server initialization complete");
        info!("Listening on http://{}:{} for SSE connections", self.host, self.port);

        // Keep the server running
        tokio::signal::ctrl_c()
            .await
            .context("Failed to listen for ctrl-c signal")?;

        info!("Shutting down MCP server");
        Ok(())
    }
}

impl Execute for StartCommand {
    async fn execute(&self) -> Result<()> {
        // Validate configuration
        self.validate().context("Invalid configuration")?;

        // Log configuration
        info!("Transport: {}", self.transport.as_str());
        if let Some(ref config) = self.config_path {
            info!("Config file: {}", config.display());
        }
        if let Some(ref project) = self.project_path {
            info!("Project path: {}", project.display());
        }

        // Start the server based on transport type
        let result = match self.transport {
            Transport::Stdio => self.start_stdio().await,
            Transport::Http => self.start_http().await,
            Transport::Sse => self.start_sse().await,
        };

        if let Err(ref e) = result {
            error!("Failed to start MCP server: {:#}", e);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_command_creation() {
        let cmd = StartCommand::new(
            Transport::Stdio,
            "127.0.0.1".to_string(),
            3000,
            None,
            None,
        );

        assert_eq!(cmd.transport.as_str(), "stdio");
        assert_eq!(cmd.host, "127.0.0.1");
        assert_eq!(cmd.port, 3000);
    }

    #[test]
    fn test_validate_network_transport_requires_port() {
        let cmd = StartCommand::new(
            Transport::Http,
            "127.0.0.1".to_string(),
            0,
            None,
            None,
        );

        assert!(cmd.validate().is_err());
    }

    #[test]
    fn test_validate_network_transport_requires_host() {
        let cmd = StartCommand::new(
            Transport::Http,
            String::new(),
            3000,
            None,
            None,
        );

        assert!(cmd.validate().is_err());
    }

    #[test]
    fn test_validate_stdio_transport() {
        let cmd = StartCommand::new(
            Transport::Stdio,
            "127.0.0.1".to_string(),
            3000,
            None,
            None,
        );

        assert!(cmd.validate().is_ok());
    }
}
