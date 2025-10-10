# Serena MCP Setup Guide for Claude Desktop

This guide documents how to configure Serena as an MCP (Model Context Protocol) server for use with Claude Desktop or other MCP clients.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Configuration](#configuration)
4. [Verification](#verification)
5. [Usage Examples](#usage-examples)
6. [Troubleshooting](#troubleshooting)
7. [Advanced Configuration](#advanced-configuration)

---

## Prerequisites

### Required Software

1. **Python 3.11** (Serena requires >=3.11, <3.12)
   ```powershell
   python --version  # Should show 3.11.x
   ```

2. **uv package manager**
   - Windows: Already installed at `C:\Users\david\bin\uv.exe`
   - Verify: `uv --version`

3. **Claude Desktop** or another MCP-compatible client

### Environment Setup

Ensure `uv` is in your PATH:
```powershell
# Check if uv is accessible
where.exe uv

# Should show: C:\Users\david\bin\uv.exe or similar
```

---

## Installation

### 1. Clone/Update Serena Repository

```powershell
# Navigate to projects directory
cd T:\projects

# If not already cloned
git clone https://github.com/oraios/serena.git serena-source

# If already cloned, update it
cd serena-source
git pull
```

### 2. Install Serena Dependencies

Serena uses `uv` for dependency management. The installation happens automatically when first run via `uvx`, but you can pre-install:

```powershell
cd T:\projects\serena-source

# Install in development mode
uv pip install -e .

# Verify installation
uv run serena --help
```

### 3. Create Global Configuration

```powershell
# Create/edit global config
uv run serena config edit

# Or manually create at: C:\Users\david\.serena\serena_config.yml
```

**Recommended `serena_config.yml` settings**:
```yaml
# GUI options (Windows)
gui_log_window: false          # Set to true if you want a separate log window
web_dashboard: true            # Recommended for monitoring
web_dashboard_open_on_launch: false  # Set to true to auto-open browser
log_level: 20                  # 20=INFO, 30=WARNING (less verbose)

# Performance
tool_timeout: 240              # 4 minutes default timeout
default_max_tool_answer_chars: 150000

# LSP settings
trace_lsp_communication: false # Set to true only for debugging

# Tool statistics (optional)
record_tool_usage_stats: false

# Token estimation (if stats enabled)
token_count_estimator: TIKTOKEN_GPT4O

# Projects will be auto-registered here
projects: []
```

---

## Configuration

### MCP Configuration File

The MCP configuration file is located at:
```
C:\Users\david\mcp.json
```

### Serena MCP Entry (Current Configuration)

```json
{
  "mcpServers": {
    "serena": {
      "command": "uvx",
      "args": [
        "--from",
        "T:/projects/serena-source",
        "serena-mcp-server",
        "--project",
        "T:/projects/serena-source",
        "--log-level",
        "WARNING"
      ],
      "env": {
        "MCP_PROTOCOL_VERSION": "2025-06-18",
        "PYTHONUNBUFFERED": "1"
      }
    }
  }
}
```

### Configuration Breakdown

| Component | Purpose |
|-----------|---------|
| `"command": "uvx"` | Uses uvx to run Serena (handles Python environment) |
| `"--from T:/projects/serena-source"` | Points to Serena source directory |
| `"serena-mcp-server"` | Entry point script (defined in pyproject.toml) |
| `"--project T:/projects/serena-source"` | Default project to activate on startup |
| `"--log-level WARNING"` | Reduces log verbosity (OPTIONS: DEBUG, INFO, WARNING, ERROR) |
| `"PYTHONUNBUFFERED": "1"` | Ensures real-time log output |

### Alternative Configurations

#### 1. Using Different Project

```json
"serena": {
  "command": "uvx",
  "args": [
    "--from",
    "T:/projects/serena-source",
    "serena-mcp-server",
    "--project",
    "C:/path/to/your/project",  // ← Change this
    "--log-level",
    "WARNING"
  ],
  "env": {
    "MCP_PROTOCOL_VERSION": "2025-06-18",
    "PYTHONUNBUFFERED": "1"
  }
}
```

#### 2. With Debug Logging

```json
"serena": {
  "command": "uvx",
  "args": [
    "--from",
    "T:/projects/serena-source",
    "serena-mcp-server",
    "--project",
    "T:/projects/serena-source",
    "--log-level",
    "DEBUG",  // ← More verbose
    "--enable-web-dashboard",
    "true",
    "--trace-lsp-communication",  // ← LSP debugging
    "true"
  ],
  "env": {
    "MCP_PROTOCOL_VERSION": "2025-06-18",
    "PYTHONUNBUFFERED": "1"
  }
}
```

#### 3. With Custom Context and Modes

```json
"serena": {
  "command": "uvx",
  "args": [
    "--from",
    "T:/projects/serena-source",
    "serena-mcp-server",
    "--project",
    "T:/projects/serena-source",
    "--context",
    "agent",
    "--mode",
    "editing",
    "--mode",
    "interactive",
    "--log-level",
    "INFO"
  ],
  "env": {
    "MCP_PROTOCOL_VERSION": "2025-06-18",
    "PYTHONUNBUFFERED": "1"
  }
}
```

#### 4. Multiple Serena Instances (Different Projects)

```json
{
  "mcpServers": {
    "serena-frontend": {
      "command": "uvx",
      "args": [
        "--from", "T:/projects/serena-source",
        "serena-mcp-server",
        "--project", "C:/projects/my-frontend",
        "--log-level", "WARNING"
      ],
      "env": {
        "MCP_PROTOCOL_VERSION": "2025-06-18",
        "PYTHONUNBUFFERED": "1"
      }
    },
    "serena-backend": {
      "command": "uvx",
      "args": [
        "--from", "T:/projects/serena-source",
        "serena-mcp-server",
        "--project", "C:/projects/my-backend",
        "--log-level", "WARNING"
      ],
      "env": {
        "MCP_PROTOCOL_VERSION": "2025-06-18",
        "PYTHONUNBUFFERED": "1"
      }
    }
  }
}
```

---

## Verification

### 1. Test Serena Command Line

```powershell
# Test that serena is accessible
uvx --from T:/projects/serena-source serena --help

# Should show Serena CLI help
```

### 2. Test MCP Server Startup

```powershell
# Start MCP server manually (for testing)
uvx --from T:/projects/serena-source serena-mcp-server --project T:/projects/serena-source --log-level DEBUG

# You should see:
# - Language server starting
# - MCP server initializing
# - Tool registration messages
# - "Server running..." or similar

# Press Ctrl+C to stop
```

### 3. Check Logs

After Claude Desktop starts Serena, check logs:

```powershell
# View latest MCP server log
ls -Sort LastWriteTime C:\Users\david\.serena\logs\mcp_*.log | Select-Object -First 1 | Get-Content -Tail 50

# Or open in notepad
notepad (ls -Sort LastWriteTime C:\Users\david\.serena\logs\mcp_*.log | Select-Object -First 1).FullName
```

### 4. Check Web Dashboard (if enabled)

If `web_dashboard: true` in config:
- Open browser to: http://localhost:24282/dashboard/
- You should see live logs and session information
- If port 24282 is in use, try 24283, 24284, etc.

---

## Usage Examples

### Basic Project Operations

Once Serena is running in Claude Desktop, you can use it:

#### 1. List Available Tools

```
Claude, please list all available Serena tools.
```

#### 2. Activate a Different Project

```
Claude, activate the project at C:/projects/my-app
```

#### 3. Search for Symbols

```
Claude, find all classes named "UserController"
```

#### 4. Read File

```
Claude, read the file src/main.py
```

#### 5. Get Symbol Overview

```
Claude, show me an overview of symbols in src/api/routes.py
```

#### 6. Search for Pattern

```
Claude, search for "TODO" in the project
```

### Advanced Operations

#### Project Onboarding

```
Claude, perform onboarding on this project to understand its structure.
```

#### Execute Shell Command

```
Claude, run "pytest tests/" to execute tests
```

#### Symbol Navigation

```
Claude, find all references to the function "calculate_total"
```

#### Memory Management

```
Claude, write a memory named "api-architecture" with: [description]
Claude, read the memory "api-architecture"
Claude, list all memories
```

---

## Troubleshooting

### Problem: "Command not found: uvx"

**Solution**: Ensure `uv` is in PATH
```powershell
$env:PATH += ";C:\Users\david\bin"
# Or add permanently via System Environment Variables
```

### Problem: "No module named serena"

**Solution**: Install Serena in uv environment
```powershell
cd T:\projects\serena-source
uv pip install -e .
```

### Problem: "Language server failed to start"

**Solution**: Check language server installation
```powershell
# For Python projects
pip install pyright

# For TypeScript projects
npm install -g typescript-language-server typescript

# Run health check
uvx --from T:/projects/serena-source serena project health-check T:/projects/serena-source
```

### Problem: "Tool timeout exceeded"

**Solution**: Increase timeout in config
```yaml
# In C:\Users\david\.serena\serena_config.yml
tool_timeout: 600  # 10 minutes instead of 4
```

Or in MCP config:
```json
"args": [
  "--from", "T:/projects/serena-source",
  "serena-mcp-server",
  "--project", "T:/projects/serena-source",
  "--tool-timeout", "600"
]
```

### Problem: "MCP server not responding"

**Solutions**:

1. **Check if running**:
   ```powershell
   Get-Process | Where-Object {$_.ProcessName -like "*python*"}
   ```

2. **Kill and restart Claude Desktop**:
   ```powershell
   Stop-Process -Name "Claude" -Force
   # Then restart Claude Desktop
   ```

3. **Check logs**:
   ```powershell
   Get-Content -Tail 100 (ls -Sort LastWriteTime C:\Users\david\.serena\logs\mcp_*.log | Select-Object -First 1).FullName
   ```

### Problem: "Project not found"

**Solution**: Register project in config
```powershell
# Edit global config
uvx --from T:/projects/serena-source serena config edit

# Add to projects section:
projects:
  - name: my-project
    root: C:/projects/my-project
```

### Problem: "Symbols not found / Slow symbol search"

**Solution**: Index the project
```powershell
uvx --from T:/projects/serena-source serena project index C:/projects/my-project
```

---

## Advanced Configuration

### Custom Modes

Create custom modes to control tool availability:

```powershell
# Create new mode
uvx --from T:/projects/serena-source serena mode create --name my-mode

# Edit mode
uvx --from T:/projects/serena-source serena mode edit my-mode
```

**Example mode file** (`~/.serena/modes/my-mode.yml`):
```yaml
name: my-mode
prompt: |
  Focus on code review and analysis.
  Do not execute any shell commands.

include_tools:
  - find_symbol
  - read_file
  - get_symbols_overview
  - search_for_pattern

exclude_tools:
  - execute_shell_command
  - create_text_file
```

**Use in MCP config**:
```json
"args": [
  "--from", "T:/projects/serena-source",
  "serena-mcp-server",
  "--project", "T:/projects/serena-source",
  "--mode", "my-mode"
]
```

### Custom Contexts

```powershell
# Create custom context
uvx --from T:/projects/serena-source serena context create --name my-context

# Edit context
uvx --from T:/projects/serena-source serena context edit my-context
```

### Project-Specific Configuration

Create `.serena/project.yml` in your project:

```powershell
cd C:/projects/my-project
mkdir .serena
uvx --from T:/projects/serena-source serena project generate-yml
```

**Edit `.serena/project.yml`**:
```yaml
language: python  # or typescript, java, etc.
project_name: "my-project"
encoding: utf-8

# Exclude certain tools for this project
excluded_tools:
  - execute_shell_command

# Initial prompt for context
initial_prompt: |
  This is a FastAPI web application.
  Main entry point: src/main.py
  
  Key directories:
  - src/api/: REST endpoints
  - src/models/: Data models
  - src/services/: Business logic
  - tests/: Unit and integration tests

# Read-only mode (no file modifications)
read_only: false

# Additional patterns to ignore
ignored_paths:
  - "*.generated.py"
  - "migrations/"
```

### Language Server Settings

Configure language-server-specific options in `serena_config.yml`:

```yaml
ls_specific_settings:
  pyright:
    python.analysis.typeCheckingMode: "strict"
    python.analysis.useLibraryCodeForTypes: true
  
  typescript-language-server:
    javascript.suggest.autoImports: true
    typescript.suggest.autoImports: true
```

### Environment-Specific Configurations

#### Development Environment
```json
"serena-dev": {
  "command": "uvx",
  "args": [
    "--from", "T:/projects/serena-source",
    "serena-mcp-server",
    "--project", "C:/projects/my-app",
    "--log-level", "DEBUG",
    "--enable-web-dashboard", "true",
    "--trace-lsp-communication", "true"
  ],
  "env": {
    "MCP_PROTOCOL_VERSION": "2025-06-18",
    "PYTHONUNBUFFERED": "1"
  }
}
```

#### Production Environment
```json
"serena-prod": {
  "command": "uvx",
  "args": [
    "--from", "T:/projects/serena-source",
    "serena-mcp-server",
    "--project", "C:/projects/my-app",
    "--log-level", "WARNING",
    "--mode", "planning",
    "--mode", "one-shot"
  ],
  "env": {
    "MCP_PROTOCOL_VERSION": "2025-06-18",
    "PYTHONUNBUFFERED": "1"
  }
}
```

---

## Performance Optimization

### 1. Pre-Index Projects

Index symbols before using Serena:
```powershell
uvx --from T:/projects/serena-source serena project index C:/projects/my-project
```

This creates a cache in `.serena/lsp-cache/` for faster symbol lookups.

### 2. Adjust Tool Timeout

For large projects or slow operations:
```yaml
# In serena_config.yml
tool_timeout: 600  # 10 minutes
```

### 3. Ignore Unnecessary Files

In `.serena/project.yml`:
```yaml
ignored_paths:
  - "node_modules/**"
  - "*.min.js"
  - "build/**"
  - "dist/**"
  - ".git/**"
```

### 4. Disable Verbose Logging

```json
"args": [
  "--log-level", "ERROR"  // Only show errors
]
```

---

## Security Considerations

### 1. Read-Only Mode

For sensitive projects, enable read-only mode:
```yaml
# In .serena/project.yml
read_only: true
```

### 2. Restrict Shell Commands

Exclude the shell command tool:
```yaml
# In .serena/project.yml
excluded_tools:
  - execute_shell_command
```

### 3. Limit Project Access

Only register projects that should be accessible:
```yaml
# In serena_config.yml
projects:
  - name: safe-project
    root: C:/projects/safe-project
```

---

## Integration with Other Tools

### 1. Git Integration

Serena works seamlessly with Git:
```
Claude, search for all files changed in the last commit
Claude, find all TODOs in modified files
```

### 2. Testing Frameworks

```
Claude, run pytest tests/ and analyze failures
Claude, find all test files for the UserService class
```

### 3. Documentation Generation

```
Claude, read all API endpoint files and create documentation
Claude, find all public classes and their docstrings
```

---

## Updating Serena

### Update from Git

```powershell
cd T:\projects\serena-source
git pull origin main

# Reinstall dependencies (if needed)
uv pip install -e .
```

### Update Configuration

After updating, check for new config options:
```powershell
# Compare with template
fc C:\Users\david\.serena\serena_config.yml T:\projects\serena-source\src\serena\resources\serena_config.template.yml
```

---

## Related Documentation

- **[CLI Usage Guide](./CLI_USAGE.md)** - Complete CLI command reference
- **[Custom Tools Guide](./CUSTOM_TOOLS.md)** - Creating custom Serena tools
- **[Architecture Documentation](./ARCHITECTURE.md)** - Serena's internal architecture

---

## Support & Resources

- **GitHub Repository**: https://github.com/oraios/serena
- **Issues**: https://github.com/oraios/serena/issues
- **Logs Location**: `C:\Users\david\.serena\logs\`
- **Dashboard**: http://localhost:24282/dashboard/ (when enabled)

---

**Configuration Last Updated**: January 10, 2025  
**Serena Version**: 0.1.4  
**User**: david  
**System**: Windows (PowerShell)
