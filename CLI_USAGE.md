# Serena CLI Usage Guide

This guide documents all CLI commands available in Serena, providing alternatives to the MCP server for project management, configuration, and development workflows.

## Table of Contents

1. [Overview](#overview)
2. [Installation & Setup](#installation--setup)
3. [Top-Level Commands](#top-level-commands)
4. [Mode Management](#mode-management)
5. [Context Management](#context-management)
6. [Project Management](#project-management)
7. [Configuration Management](#configuration-management)
8. [Tool Management](#tool-management)
9. [Prompt Management](#prompt-management)
10. [Common Workflows](#common-workflows)

---

## Overview

Serena provides a comprehensive CLI powered by Click, offering commands for:

- **Server Management**: Start MCP server with various options
- **Project Operations**: Initialize, index, and validate projects
- **Configuration**: Manage global and project settings
- **Customization**: Create and edit modes, contexts, and prompts
- **Development**: Health checks, debugging, and testing

### Command Structure

```
serena <command> [subcommand] [options] [arguments]
```

**Top-level groups**:
- `serena start-mcp-server` - Start the MCP server
- `serena print-system-prompt` - Generate system prompts
- `serena mode` - Manage modes
- `serena context` - Manage contexts
- `serena project` - Manage projects
- `serena config` - Manage configuration
- `serena tools` - Manage tools
- `serena prompts` - Manage prompts

---

## Installation & Setup

### Installing Serena

```bash
# Install from source
cd serena-source
uv pip install -e .

# Verify installation
serena --help
```

### Initial Configuration

```bash
# Create default configuration file
serena config edit

# Generate project.yml for a project
cd /path/to/your/project
serena project generate-yml
```

---

## Top-Level Commands

### 1. Start MCP Server

**Command**: `serena start-mcp-server`

Start the Serena MCP (Model Context Protocol) server for integration with LLM clients like Claude Desktop.

#### Basic Usage

```bash
# Start with default settings
serena start-mcp-server

# Start with a specific project
serena start-mcp-server --project /path/to/project

# Start with project by name (from config)
serena start-mcp-server --project my-project-name
```

#### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--project` | path/name | None | Project to activate at startup |
| `--context` | string | "agent" | Built-in context or path to custom context YAML |
| `--mode` | string | ["editing", "interactive"] | Mode names (can specify multiple) |
| `--transport` | choice | "stdio" | Transport protocol: stdio, sse, streamable-http |
| `--host` | string | "0.0.0.0" | Host for HTTP transports |
| `--port` | int | 8000 | Port for HTTP transports |
| `--enable-web-dashboard` | bool | None | Override dashboard setting |
| `--enable-gui-log-window` | bool | None | Override GUI log window setting |
| `--log-level` | choice | None | Log level: DEBUG, INFO, WARNING, ERROR, CRITICAL |
| `--trace-lsp-communication` | bool | None | Trace language server communication |
| `--tool-timeout` | float | None | Tool execution timeout in seconds |

#### Examples

```bash
# Development mode with debugging
serena start-mcp-server \
  --project ./my-project \
  --log-level DEBUG \
  --trace-lsp-communication true \
  --enable-web-dashboard true

# Production mode, planning context
serena start-mcp-server \
  --project prod-app \
  --context planning \
  --mode planning --mode one-shot \
  --log-level WARNING

# HTTP transport for network access
serena start-mcp-server \
  --transport sse \
  --host 0.0.0.0 \
  --port 8080 \
  --project my-project
```

### 2. Print System Prompt

**Command**: `serena print-system-prompt`

Generate and display the system prompt that would be used for a project. Useful for understanding what instructions the LLM receives.

#### Usage

```bash
# Print full prompt for current directory
serena print-system-prompt

# Print prompt for specific project
serena print-system-prompt /path/to/project

# Print only instructions (no prefix/postfix)
serena print-system-prompt --only-instructions

# Print with specific context and modes
serena print-system-prompt \
  --context agent \
  --mode editing --mode interactive \
  --log-level WARNING
```

#### Options

| Option | Description |
|--------|-------------|
| `--log-level` | Log level for prompt generation (default: WARNING) |
| `--only-instructions` | Print only instructions, without wrapper text |
| `--context` | Context to use (default: "agent") |
| `--mode` | Modes to activate (can specify multiple) |

---

## Mode Management

**Command Group**: `serena mode`

Modes modify tool availability and behavior. They allow fine-grained control over what Serena can do.

### List Modes

```bash
# List all available modes
serena mode list
```

**Output Example**:
```
editing      (internal)
interactive  (internal)
planning     (internal)
one-shot     (internal)
my-mode      (at /home/user/.serena/modes/my-mode.yml)
```

### Create Mode

```bash
# Create new mode from template
serena mode create --name my-custom-mode

# Copy and customize an internal mode
serena mode create --from-internal editing

# Copy internal mode with same name (overrides it)
serena mode create --from-internal editing
```

Creates a new mode YAML file in `~/.serena/modes/` and opens it in your default editor.

**Mode YAML Structure**:
```yaml
name: my-custom-mode
prompt: |
  Additional instructions for this mode.
  
  The agent should focus on X and Y.

include_tools:
  - find_symbol
  - read_file
  
exclude_tools:
  - execute_shell_command
```

### Edit Mode

```bash
# Edit a custom mode
serena mode edit my-custom-mode
```

Opens the mode file in your default editor. Only works for custom modes (not internal).

### Delete Mode

```bash
# Delete a custom mode
serena mode delete my-custom-mode
```

---

## Context Management

**Command Group**: `serena context`

Contexts define the operational environment and available tools. They're similar to modes but more fundamental.

### List Contexts

```bash
# List all available contexts
serena context list
```

**Built-in Contexts**:
- `agent`: General-purpose coding agent
- `ide_assistant`: Optimized for IDE integration

### Create Context

```bash
# Create new context from template
serena context create --name my-context

# Copy an internal context
serena context create --from-internal agent
```

Creates a new context YAML file in `~/.serena/contexts/`.

**Context YAML Structure**:
```yaml
name: my-context
prompt: |
  You are a specialized agent for...
  
  Focus on...

include_tools:
  - "*"  # All tools
  
exclude_tools:
  - dangerous_tool

tool_description_overrides:
  find_symbol: "Custom description for find_symbol tool"
```

### Edit Context

```bash
# Edit a custom context
serena context edit my-context
```

### Delete Context

```bash
# Delete a custom context
serena context delete my-context
```

---

## Project Management

**Command Group**: `serena project`

Manage Serena projects, including initialization, indexing, and health checks.

### Generate Project Configuration

**Command**: `serena project generate-yml`

Create a `project.yml` configuration file for a project.

```bash
# Generate for current directory
cd /path/to/project
serena project generate-yml

# Generate for specific directory
serena project generate-yml /path/to/other/project

# Specify language explicitly
serena project generate-yml --language python
serena project generate-yml --language typescript
```

**Generated `project.yml` Example**:
```yaml
language: python
name: my-project
encoding: utf-8

# Optionally customize
include_tools:
  - "*"

exclude_tools:
  - execute_shell_command

read_only: false

initial_prompt: |
  This project implements...
```

### Index Project

**Command**: `serena project index`

Pre-index all symbols in a project for faster symbol lookups.

```bash
# Index current directory
serena project index

# Index specific project
serena project index /path/to/project

# Index with debugging
serena project index --log-level DEBUG

# Set timeout for each file
serena project index --timeout 30
```

**What it does**:
- Starts language server
- Requests symbols for all source files
- Saves symbols to LSP cache (`.serena/lsp-cache/`)
- Shows progress bar
- Reports any failures

**When to use**:
- After cloning a new repository
- After major refactoring
- Before starting work on a large codebase
- When symbol search is slow

### Index Single File

**Command**: `serena project index-file`

Index a single file (useful for debugging).

```bash
# Index a specific file
serena project index-file src/main.py

# Index with verbose output
serena project index-file src/main.py --verbose

# Index file in different project
serena project index-file src/main.py /path/to/project
```

### Check Ignored Path

**Command**: `serena project is_ignored_path`

Check if a path is ignored by the project configuration.

```bash
# Check if path is ignored (current project)
serena project is_ignored_path node_modules/package.json

# Check in specific project
serena project is_ignored_path build/output.js /path/to/project
```

**Output**:
```
Path 'node_modules/package.json' IS ignored by the project configuration.
```

### Health Check

**Command**: `serena project health-check`

Perform comprehensive health check of project tools and language server.

```bash
# Check current directory
serena project health-check

# Check specific project
serena project health-check /path/to/project
```

**What it tests**:
1. SerenaAgent initialization
2. Finding source files
3. `GetSymbolsOverviewTool`
4. `FindSymbolTool`
5. `FindReferencingSymbolsTool`
6. `SearchForPatternTool`

**Output**:
```
✅ Health check passed - All tools working correctly
Log saved to: /path/to/project/.serena/logs/health-checks/health_check_20250110_163000.log
```

---

## Configuration Management

**Command Group**: `serena config`

### Edit Global Configuration

**Command**: `serena config edit`

Open the global Serena configuration file in your default editor.

```bash
serena config edit
```

**Configuration File Location**: `~/.serena/serena_config.yml`

**Configuration Options**:
```yaml
# Global configuration
web_dashboard: true
web_dashboard_open_on_launch: false
gui_log_window_enabled: false

log_level: 20  # INFO
trace_lsp_communication: false

tool_timeout: 300  # seconds
default_max_tool_answer_chars: 50000

record_tool_usage_stats: false
token_count_estimator: anthropic_conservative

# JetBrains plugin mode (vs LSP)
jetbrains: false

# Registered projects
projects:
  - name: my-project
    root: /path/to/my-project
  - name: another-project
    root: /path/to/another-project

# Language server specific settings
ls_specific_settings:
  pyright:
    python.analysis.typeCheckingMode: "basic"
  typescript-language-server:
    javascript.suggest.autoImports: true
```

---

## Tool Management

**Command Group**: `serena tools`

List and describe available tools.

### List Tools

**Command**: `serena tools list`

```bash
# List default-enabled tools
serena tools list

# List all tools (including optional)
serena tools list --all

# List only optional tools
serena tools list --only-optional

# Quiet mode (names only, for scripting)
serena tools list --quiet
serena tools list --all --quiet
```

**Output Example**:
```
 * `find_symbol`: Performs a global search for symbols
 * `read_file`: Reads a file within the project directory
 * `get_symbols_overview`: Gets an overview of symbols in a file
 * `create_text_file`: Creates/overwrites a file
 ...
```

### Get Tool Description

**Command**: `serena tools description`

Get detailed description of a specific tool.

```bash
# Get tool description
serena tools description find_symbol

# Get description with specific context
serena tools description find_symbol --context agent
```

**Output**:
```
Retrieves information on all symbols/code entities (classes, methods, etc.) 
based on the given `name_path`, which represents a pattern for the symbol's 
path within the symbol tree of a single file.
...
```

---

## Prompt Management

**Command Group**: `serena prompts`

Customize Serena's internal prompts by creating overrides.

### List Prompts

```bash
# List all prompt YAML files
serena prompts list
```

**Output Example**:
```
initial_instructions.yml
onboarding.yml
tool_descriptions.yml
thinking_prompts.yml
```

### Create Prompt Override

```bash
# Create an override for customization
serena prompts create-override initial_instructions.yml
serena prompts create-override onboarding
```

Creates a copy in `~/.serena/prompt_templates/` and opens it for editing.

### Edit Prompt Override

```bash
# Edit existing override
serena prompts edit-override initial_instructions.yml
```

### List Overrides

```bash
# List all active prompt overrides
serena prompts list-overrides
```

### Delete Prompt Override

```bash
# Delete an override (reverts to default)
serena prompts delete-override initial_instructions.yml
```

---

## Common Workflows

### 1. Setting Up a New Project

```bash
# Navigate to project
cd /path/to/new/project

# Generate project.yml
serena project generate-yml --language python

# Edit configuration if needed
vim .serena/project.yml

# Index the project for fast symbol lookup
serena project index

# Run health check
serena project health-check

# Start MCP server with the project
serena start-mcp-server --project .
```

### 2. Creating Custom Development Mode

```bash
# Create a custom mode for your workflow
serena mode create --name my-dev-mode

# Edit the mode file
# Add tools, prompts, etc.

# Test it
serena start-mcp-server \
  --project my-project \
  --mode my-dev-mode \
  --log-level DEBUG
```

### 3. Debugging Symbol Issues

```bash
# Check if file is being indexed
serena project index-file src/problematic.py --verbose

# Check if path is ignored
serena project is_ignored_path src/problematic.py

# Re-index entire project
serena project index --log-level DEBUG

# Run health check
serena project health-check
```

### 4. Customizing for Your Team

```bash
# Create team-specific context
serena context create --name team-context

# Create team-specific mode
serena mode create --name team-mode

# Override prompts
serena prompts create-override initial_instructions.yml

# Share these files with team
# Files are in ~/.serena/contexts/, ~/.serena/modes/, ~/.serena/prompt_templates/
```

### 5. Multiple Projects

```bash
# Edit global config to register projects
serena config edit

# Add to config:
# projects:
#   - name: frontend
#     root: /path/to/frontend
#   - name: backend
#     root: /path/to/backend

# Start server for specific project by name
serena start-mcp-server --project frontend
serena start-mcp-server --project backend
```

### 6. Development vs Production

```bash
# Development: All features, verbose logging
serena start-mcp-server \
  --project dev-project \
  --enable-web-dashboard true \
  --enable-gui-log-window true \
  --log-level DEBUG \
  --trace-lsp-communication true

# Production: Minimal logging, stable features
serena start-mcp-server \
  --project prod-project \
  --enable-web-dashboard false \
  --enable-gui-log-window false \
  --log-level WARNING \
  --mode planning --mode one-shot
```

### 7. Generating Documentation

```bash
# Generate system prompt for documentation
serena print-system-prompt /path/to/project > system-prompt.txt

# Get tool descriptions for documentation
serena tools list --quiet | while read tool; do
  echo "## $tool"
  serena tools description "$tool"
  echo ""
done > tools-documentation.md
```

### 8. CI/CD Integration

```bash
# In CI pipeline: Index project for cache
serena project index --log-level WARNING

# Run health check
serena project health-check || exit 1

# Generate and validate project.yml
serena project generate-yml --language python
git diff --exit-code .serena/project.yml
```

---

## Environment Variables

Serena respects several environment variables:

| Variable | Description |
|----------|-------------|
| `EDITOR` | Default editor for `edit` commands |
| `SERENA_HOME` | Override default `~/.serena` directory |
| `SERENA_LOG_LEVEL` | Default log level |

**Example**:
```bash
export EDITOR=vim
export SERENA_LOG_LEVEL=DEBUG

serena mode edit my-mode
```

---

## Configuration Files

### Locations

| File | Location | Purpose |
|------|----------|---------|
| Global config | `~/.serena/serena_config.yml` | Global settings |
| Project config | `<project>/.serena/project.yml` | Project-specific settings |
| Modes | `~/.serena/modes/*.yml` | Custom mode definitions |
| Contexts | `~/.serena/contexts/*.yml` | Custom context definitions |
| Prompt overrides | `~/.serena/prompt_templates/*.yml` | Customized prompts |
| LSP cache | `<project>/.serena/lsp-cache/` | Indexed symbols |
| Memories | `<project>/.serena/memories/` | Project knowledge |
| Logs | `~/.serena/logs/` | Application logs |

### Project.yml Structure

```yaml
# Required
language: python  # Language for LSP
name: my-project
encoding: utf-8

# Optional: Tool management
include_tools:
  - "*"  # All tools, or specific list

exclude_tools:
  - execute_shell_command

# Optional: Project info for LLM
initial_prompt: |
  This project implements a web API using FastAPI.
  The main entry point is src/main.py.
  
  Key modules:
  - src/api/: REST endpoints
  - src/models/: Data models
  - src/services/: Business logic

# Optional: Read-only mode
read_only: false

# Optional: Additional patterns to ignore
additional_ignored_patterns:
  - "*.generated.py"
  - "migrations/"

# Optional: Non-source files (for documentation, config)
non_source_files:
  - "*.md"
  - "*.yml"
```

---

## Tips & Tricks

### 1. Quick Project Setup

Create an alias for common setup:

```bash
# In ~/.bashrc or ~/.zshrc
alias serena-setup='serena project generate-yml && serena project index && serena project health-check'

# Usage
cd /path/to/new/project
serena-setup
```

### 2. Shell Completion

```bash
# Generate completion script (Bash)
_SERENA_COMPLETE=bash_source serena > ~/.serena-complete.bash
echo "source ~/.serena-complete.bash" >> ~/.bashrc

# For Zsh
_SERENA_COMPLETE=zsh_source serena > ~/.serena-complete.zsh
echo "source ~/.serena-complete.zsh" >> ~/.zshrc
```

### 3. Viewing Live Logs

```bash
# Start server in background
serena start-mcp-server --project my-project &

# Find log file
ls -lt ~/.serena/logs/mcp_*.log | head -1

# Tail logs
tail -f ~/.serena/logs/mcp_TIMESTAMP.log
```

### 4. Scripting with Serena

```bash
#!/bin/bash
# deploy.sh - Deployment script with health check

echo "Running health check..."
if ! serena project health-check; then
    echo "Health check failed!"
    exit 1
fi

echo "Indexing project..."
serena project index --log-level WARNING

echo "Starting deployment..."
# ... your deployment commands
```

---

## Troubleshooting

### Common Issues

**Problem**: "Language server failed to start"
```bash
# Solution: Check language server installation
serena project health-check

# Check logs
tail -n 100 ~/.serena/logs/mcp_*.log
```

**Problem**: "No symbols found"
```bash
# Solution: Re-index project
serena project index --log-level DEBUG

# Check specific file
serena project index-file src/problem.py --verbose
```

**Problem**: "Tool not available"
```bash
# Solution: Check tool list
serena tools list --all

# Check project configuration
cat .serena/project.yml

# Check mode configuration
serena mode list
```

**Problem**: "Path is ignored"
```bash
# Solution: Check if path is ignored
serena project is_ignored_path path/to/file.py

# Edit project.yml to adjust ignored patterns
```

---

## Advanced Usage

### Custom Shell Integration

**Example**: ZSH function for quick mode switching

```bash
# In ~/.zshrc
serena-mode() {
    local mode="${1:-editing}"
    serena start-mcp-server \
        --project "$(pwd)" \
        --mode "$mode" \
        --log-level WARNING
}

# Usage
serena-mode planning
serena-mode interactive
```

### Programmatic Access

The CLI is built with Click, so you can also import and use it programmatically:

```python
from serena.cli import project

# Call CLI commands from Python
from click.testing import CliRunner

runner = CliRunner()
result = runner.invoke(project.index, ['/path/to/project'])
print(result.output)
```

---

## Next Steps

- **Python Library API**: See `PYTHON_LIBRARY_API.md` for programmatic usage
- **Custom Tools**: See `CUSTOM_TOOLS.md` for creating custom tools
- **Configuration**: See `CONFIGURATION.md` for detailed settings (TBD)
- **Dashboard**: See `DASHBOARD_API.md` for web dashboard (TBD)

---

**Last Updated**: January 10, 2025
