use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use serena_config::{loader::ConfigLoader, ProjectConfig, SerenaConfig};
use serena_core::ToolRegistry;
use serena_lsp::LanguageServerManager;
use serena_mcp::SerenaMcpServer;

/// Main application structure that manages the Serena lifecycle
pub struct App {
    /// MCP server instance (consumed when starting)
    mcp_server: Option<SerenaMcpServer>,

    /// LSP manager for language servers
    lsp_manager: Arc<LanguageServerManager>,

    /// Tool registry
    tool_registry: Arc<ToolRegistry>,

    /// Application configuration
    config: Arc<RwLock<SerenaConfig>>,

    /// Active project configuration
    project_config: Arc<RwLock<Option<ProjectConfig>>>,

    /// Configuration loader
    config_loader: ConfigLoader,
}

impl App {
    /// Create a new App instance
    pub async fn new(
        config_path: Option<PathBuf>,
        project_path: Option<PathBuf>,
    ) -> Result<Self> {
        info!("Initializing Serena application");

        // Initialize configuration loader
        let config_loader = ConfigLoader::new();

        // Load configuration
        let config = Self::load_config(&config_loader, config_path).await?;
        let config = Arc::new(RwLock::new(config));

        // Initialize tool registry
        debug!("Initializing tool registry");
        let tool_registry = Arc::new(ToolRegistry::new());

        // Determine root path for LSP manager
        let root_path = project_path
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        // Initialize LSP manager
        debug!("Initializing LSP manager with root: {}", root_path.display());
        let lsp_manager = Arc::new(LanguageServerManager::new(root_path));

        // Initialize MCP server with tool registry
        debug!("Initializing MCP server");
        let mcp_server = Some(SerenaMcpServer::new((*tool_registry).clone()));

        // Load project if specified
        let project_config = if let Some(project_path) = project_path {
            info!("Loading project from: {}", project_path.display());
            let proj_config = Self::load_project_config(&project_path)?;
            Arc::new(RwLock::new(Some(proj_config)))
        } else {
            Arc::new(RwLock::new(None))
        };

        info!("Serena application initialized successfully");

        Ok(Self {
            mcp_server,
            lsp_manager,
            tool_registry,
            config,
            project_config,
            config_loader,
        })
    }

    /// Load configuration from file or use defaults
    async fn load_config(
        loader: &ConfigLoader,
        config_path: Option<PathBuf>,
    ) -> Result<SerenaConfig> {
        if let Some(path) = config_path {
            info!("Loading configuration from: {}", path.display());
            loader
                .load_from_file(&path)
                .context("Failed to load configuration file")
        } else {
            info!("Loading configuration from default locations");
            loader
                .load()
                .context("Failed to load configuration")
        }
    }

    /// Load project configuration from a directory
    fn load_project_config(project_path: &PathBuf) -> Result<ProjectConfig> {
        // Try to find and load project.yml in the project directory
        let loader = ConfigLoader::new();

        if let Some(config_file) = loader.find_project_config(project_path) {
            info!("Found project config: {}", config_file.display());
            let content = std::fs::read_to_string(&config_file)?;
            let mut config: ProjectConfig = serde_yaml::from_str(&content)?;
            config.root = project_path.clone();
            Ok(config)
        } else {
            // Create a default project config
            info!("No project config found, creating default");
            let project_name = project_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unnamed")
                .to_string();

            let mut config = ProjectConfig::new(project_name, project_path.clone());

            // Try to detect languages
            if let Err(e) = config.detect_languages() {
                warn!("Failed to detect languages: {}", e);
            }

            Ok(config)
        }
    }

    /// Set the operating mode
    pub fn set_mode(&mut self, mode: &str) -> Result<()> {
        info!("Setting mode to: {}", mode);
        // TODO: Implement mode switching when serena-config supports it
        // For now, just log the request
        Ok(())
    }

    /// Set the context
    pub fn set_context(&mut self, context: &str) -> Result<()> {
        info!("Setting context to: {}", context);
        // TODO: Implement context switching when serena-config supports it
        // For now, just log the request
        Ok(())
    }

    /// Run the MCP server using stdio transport
    pub async fn run_stdio(mut self) -> Result<()> {
        info!("Running MCP server on stdio transport");
        
        let server = self.mcp_server.take()
            .ok_or_else(|| anyhow::anyhow!("MCP server already consumed"))?;
        
        server.serve_stdio().await
    }

    /// Run the MCP server using HTTP transport
    pub async fn run_http(mut self, port: u16) -> Result<()> {
        info!("Running MCP server on HTTP transport (port {})", port);
        
        let _server = self.mcp_server.take()
            .ok_or_else(|| anyhow::anyhow!("MCP server already consumed"))?;
        
        // TODO: Implement HTTP transport when available in serena-mcp
        anyhow::bail!("HTTP transport not yet implemented");
    }

    /// Run the MCP server using SSE transport
    pub async fn run_sse(mut self, port: u16) -> Result<()> {
        info!("Running MCP server on SSE transport (port {})", port);
        
        let _server = self.mcp_server.take()
            .ok_or_else(|| anyhow::anyhow!("MCP server already consumed"))?;
        
        // TODO: Implement SSE transport when available in serena-mcp
        anyhow::bail!("SSE transport not yet implemented");
    }

    /// Get the current configuration
    pub async fn get_config(&self) -> SerenaConfig {
        self.config.read().await.clone()
    }

    /// Update the configuration
    pub async fn update_config(&self, config: SerenaConfig) -> Result<()> {
        let mut cfg = self.config.write().await;
        *cfg = config;
        Ok(())
    }

    /// Get the current project configuration
    pub async fn get_project_config(&self) -> Option<ProjectConfig> {
        self.project_config.read().await.clone()
    }

    /// Activate a project
    pub async fn activate_project(&self, project_path: PathBuf) -> Result<()> {
        info!("Activating project: {}", project_path.display());

        let proj_config = Self::load_project_config(&project_path)?;

        let mut project = self.project_config.write().await;
        *project = Some(proj_config);

        // TODO: Initialize LSP servers for detected languages
        // TODO: Load project-specific tools and memory

        Ok(())
    }

    /// Deactivate the current project
    pub async fn deactivate_project(&self) -> Result<()> {
        info!("Deactivating current project");

        let mut project = self.project_config.write().await;
        *project = None;

        // TODO: Shutdown LSP servers
        // TODO: Save project state

        Ok(())
    }

    /// Shutdown the application gracefully
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down Serena application");

        // Save any pending state
        if let Some(_proj_config) = self.get_project_config().await {
            // TODO: Save project state
        }

        // Shutdown all LSP servers
        self.lsp_manager.stop_all_servers().await;

        info!("Serena application shutdown complete");
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        debug!("App instance dropped");
    }
}
