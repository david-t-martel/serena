# Serena Documentation Index

**Last Updated**: January 10, 2025  
**Serena Version**: 0.1.4

This index provides a comprehensive guide to all Serena documentation, organized by topic and use case.

---

## 📚 Quick Start

New to Serena? Start here:

1. **[MCP Setup Guide](./SERENA_MCP_SETUP.md)** - Configure Serena for Claude Desktop
2. **[MCP Config Changes](./MCP_CONFIG_CHANGES.md)** - Quick reference for recent configuration updates
3. **[CLI Usage](./CLI_USAGE.md)** - Command-line interface basics

---

## 📖 Core Documentation

### Installation & Configuration

| Document | Description | Audience |
|----------|-------------|----------|
| [SERENA_MCP_SETUP.md](./SERENA_MCP_SETUP.md) | Complete MCP server setup for Claude Desktop | All Users |
| [MCP_CONFIG_CHANGES.md](./MCP_CONFIG_CHANGES.md) | Summary of configuration changes (Jan 10, 2025) | All Users |
| [llms-install.md](./llms-install.md) | Installation instructions for AI assistants | Developers |

### Command Line Interface

| Document | Description | Audience |
|----------|-------------|----------|
| [CLI_USAGE.md](./CLI_USAGE.md) | Complete CLI command reference | All Users |
| - Server Management | Starting/stopping MCP server | Operations |
| - Project Management | Indexing, health checks, configuration | Developers |
| - Mode & Context Management | Customizing behavior | Advanced Users |
| - Tool Management | Listing and describing tools | All Users |

### Architecture & Development

| Document | Description | Audience |
|----------|-------------|----------|
| [ARCHITECTURE.md](./ARCHITECTURE.md) | Complete system architecture | Developers |
| [CUSTOM_TOOLS.md](./CUSTOM_TOOLS.md) | Creating custom tools for Serena | Developers |
| [PYTHON_LIBRARY_API.md](./PYTHON_LIBRARY_API.md) | Using Serena programmatically | Developers |

---

## 🎯 Documentation by Use Case

### For End Users (Using Serena via Claude Desktop)

**Getting Started**:
1. [SERENA_MCP_SETUP.md](./SERENA_MCP_SETUP.md#installation) - Installation
2. [SERENA_MCP_SETUP.md](./SERENA_MCP_SETUP.md#configuration) - Basic configuration
3. [SERENA_MCP_SETUP.md](./SERENA_MCP_SETUP.md#verification) - Testing setup
4. [SERENA_MCP_SETUP.md](./SERENA_MCP_SETUP.md#usage-examples) - Example commands

**Common Tasks**:
- Configure for your project: [SERENA_MCP_SETUP.md § Project-Specific Configuration](./SERENA_MCP_SETUP.md#project-specific-configuration)
- Troubleshoot issues: [SERENA_MCP_SETUP.md § Troubleshooting](./SERENA_MCP_SETUP.md#troubleshooting)
- Optimize performance: [SERENA_MCP_SETUP.md § Performance Optimization](./SERENA_MCP_SETUP.md#performance-optimization)

### For Developers (Using Serena CLI)

**Getting Started**:
1. [CLI_USAGE.md](./CLI_USAGE.md#installation--setup) - CLI installation
2. [CLI_USAGE.md](./CLI_USAGE.md#top-level-commands) - Basic commands
3. [CLI_USAGE.md](./CLI_USAGE.md#project-management) - Project operations

**Development Workflows**:
- Project setup: [CLI_USAGE.md § Setting Up a New Project](./CLI_USAGE.md#1-setting-up-a-new-project)
- Custom development modes: [CLI_USAGE.md § Creating Custom Development Mode](./CLI_USAGE.md#2-creating-custom-development-mode)
- Debugging symbols: [CLI_USAGE.md § Debugging Symbol Issues](./CLI_USAGE.md#3-debugging-symbol-issues)
- CI/CD integration: [CLI_USAGE.md § CI/CD Integration](./CLI_USAGE.md#8-cicd-integration)

### For Tool Developers (Extending Serena)

**Getting Started**:
1. [ARCHITECTURE.md](./ARCHITECTURE.md#component-overview) - Understanding the architecture
2. [CUSTOM_TOOLS.md](./CUSTOM_TOOLS.md#creating-a-basic-tool) - Tool development basics
3. [CUSTOM_TOOLS.md](./CUSTOM_TOOLS.md#registration-mechanics) - Tool registration

**Advanced Topics**:
- Tool lifecycle: [CUSTOM_TOOLS.md § Tool Lifecycle](./CUSTOM_TOOLS.md#tool-lifecycle)
- Resource access: [CUSTOM_TOOLS.md § Resource Access](./CUSTOM_TOOLS.md#resource-access)
- Design patterns: [CUSTOM_TOOLS.md § Design Patterns](./CUSTOM_TOOLS.md#design-patterns)
- Best practices: [CUSTOM_TOOLS.md § Best Practices](./CUSTOM_TOOLS.md#best-practices)

### For System Integrators (Using Serena Programmatically)

**Getting Started**:
1. [ARCHITECTURE.md](./ARCHITECTURE.md) - System architecture
2. [PYTHON_LIBRARY_API.md](./PYTHON_LIBRARY_API.md) - Python API (when available)

**Integration Patterns**:
- Embedding Serena: TBD
- Custom transports: TBD
- API authentication: TBD

---

## 🔧 Configuration Reference

### Global Configuration

**File**: `~/.serena/serena_config.yml`

**Documentation**:
- Template: [src/serena/resources/serena_config.template.yml](./src/serena/resources/serena_config.template.yml)
- Edit command: `serena config edit`
- Reference: [SERENA_MCP_SETUP.md § Configuration](./SERENA_MCP_SETUP.md#configuration)

**Key Settings**:
- `log_level`: Logging verbosity (DEBUG, INFO, WARNING, ERROR)
- `tool_timeout`: Maximum tool execution time
- `web_dashboard`: Enable web dashboard
- `trace_lsp_communication`: Debug language server
- `projects`: Registered projects list

### Project Configuration

**File**: `<project>/.serena/project.yml`

**Documentation**:
- Template: [src/serena/resources/project.template.yml](./src/serena/resources/project.template.yml)
- Generate command: `serena project generate-yml`
- Reference: [CLI_USAGE.md § Project Management](./CLI_USAGE.md#project-management)

**Key Settings**:
- `language`: Project programming language
- `excluded_tools`: Tools to disable for this project
- `initial_prompt`: Context for LLM
- `read_only`: Prevent file modifications
- `ignored_paths`: Files/directories to ignore

### MCP Configuration

**File**: `C:\Users\david\mcp.json` (Windows) or `~/.config/claude/mcp.json` (macOS/Linux)

**Documentation**:
- Current config: [MCP_CONFIG_CHANGES.md](./MCP_CONFIG_CHANGES.md)
- Setup guide: [SERENA_MCP_SETUP.md § Configuration](./SERENA_MCP_SETUP.md#configuration)
- Alternative configs: [SERENA_MCP_SETUP.md § Alternative Configurations](./SERENA_MCP_SETUP.md#alternative-configurations)

---

## 🛠️ Command Reference

### CLI Commands

Full documentation: [CLI_USAGE.md](./CLI_USAGE.md)

**Quick Reference**:

| Command | Description |
|---------|-------------|
| `serena start-mcp-server` | Start MCP server |
| `serena print-system-prompt` | Generate system prompt |
| `serena mode list` | List available modes |
| `serena mode create` | Create custom mode |
| `serena context list` | List available contexts |
| `serena project generate-yml` | Generate project config |
| `serena project index` | Index project symbols |
| `serena project health-check` | Run health diagnostics |
| `serena config edit` | Edit global config |
| `serena tools list` | List available tools |
| `serena prompts list` | List prompt templates |

### MCP Tools (via Claude)

Once Serena is running as an MCP server, you can use these operations:

**Project Operations**:
- `activate_project`: Switch to different project
- `get_current_config`: View active configuration
- `onboarding`: Analyze project structure

**Code Navigation**:
- `find_symbol`: Search for symbols globally
- `get_symbols_overview`: List file symbols
- `find_referencing_symbols`: Find references
- `find_referencing_code_snippets`: Get reference snippets

**File Operations**:
- `read_file`: Read file contents
- `list_dir`: List directory contents
- `create_text_file`: Create/overwrite files
- `insert_at_line`: Insert content at line
- `replace_lines`: Replace line ranges
- `delete_lines`: Delete line ranges

**Symbol Editing**:
- `insert_before_symbol`: Insert before symbol
- `insert_after_symbol`: Insert after symbol
- `replace_symbol_body`: Replace symbol definition

**Search**:
- `search_for_pattern`: Search files for pattern

**Memory**:
- `write_memory`: Save project knowledge
- `read_memory`: Retrieve saved knowledge
- `list_memories`: List all memories
- `delete_memory`: Remove memory

**System**:
- `execute_shell_command`: Run shell commands
- `restart_language_server`: Restart LSP
- `switch_modes`: Change active modes

---

## 🎓 Learning Path

### Beginner Path

1. **Setup** (30 min)
   - Read: [SERENA_MCP_SETUP.md](./SERENA_MCP_SETUP.md)
   - Do: Install and configure for Claude Desktop
   - Verify: Test with basic commands

2. **Basic Usage** (1 hour)
   - Read: [SERENA_MCP_SETUP.md § Usage Examples](./SERENA_MCP_SETUP.md#usage-examples)
   - Do: Try file reading, symbol search, pattern search
   - Practice: Navigate a sample project

3. **Project Setup** (30 min)
   - Read: [CLI_USAGE.md § Project Management](./CLI_USAGE.md#project-management)
   - Do: Generate project.yml for your project
   - Index: Pre-index symbols for faster lookup

### Intermediate Path

1. **CLI Mastery** (1 hour)
   - Read: [CLI_USAGE.md](./CLI_USAGE.md)
   - Do: Try all command groups
   - Script: Create shell aliases for common tasks

2. **Customization** (2 hours)
   - Read: [CLI_USAGE.md § Mode Management](./CLI_USAGE.md#mode-management)
   - Do: Create custom mode for your workflow
   - Configure: Set up project-specific configs

3. **Troubleshooting** (1 hour)
   - Read: [SERENA_MCP_SETUP.md § Troubleshooting](./SERENA_MCP_SETUP.md#troubleshooting)
   - Practice: Diagnose and fix common issues
   - Monitor: Use web dashboard for debugging

### Advanced Path

1. **Architecture** (2 hours)
   - Read: [ARCHITECTURE.md](./ARCHITECTURE.md)
   - Understand: Component interaction
   - Explore: Source code organization

2. **Tool Development** (4 hours)
   - Read: [CUSTOM_TOOLS.md](./CUSTOM_TOOLS.md)
   - Build: Simple custom tool
   - Extend: Add to existing tool

3. **Integration** (2 hours)
   - Read: [PYTHON_LIBRARY_API.md](./PYTHON_LIBRARY_API.md)
   - Integrate: Use Serena programmatically
   - Automate: Create custom workflows

---

## 🔍 Topic Index

### Architecture & Design

- Component Overview: [ARCHITECTURE.md § Component Overview](./ARCHITECTURE.md#component-overview)
- Tool System: [CUSTOM_TOOLS.md § Tool Architecture](./CUSTOM_TOOLS.md#tool-architecture)
- Language Server: [ARCHITECTURE.md § Language Server Integration](./ARCHITECTURE.md#language-server-integration)
- MCP Protocol: [ARCHITECTURE.md § MCP Protocol](./ARCHITECTURE.md#mcp-protocol)

### Configuration

- Global Config: [SERENA_MCP_SETUP.md § Global Configuration](./SERENA_MCP_SETUP.md#3-create-global-configuration)
- Project Config: [CLI_USAGE.md § Project Configuration](./CLI_USAGE.md#projectyml-structure)
- MCP Config: [SERENA_MCP_SETUP.md § MCP Configuration](./SERENA_MCP_SETUP.md#serena-mcp-entry-current-configuration)
- Modes & Contexts: [CLI_USAGE.md § Mode Management](./CLI_USAGE.md#mode-management)

### Development

- Tool Development: [CUSTOM_TOOLS.md](./CUSTOM_TOOLS.md)
- Python API: [PYTHON_LIBRARY_API.md](./PYTHON_LIBRARY_API.md)
- Contributing: TBD

### Operations

- Installation: [SERENA_MCP_SETUP.md § Installation](./SERENA_MCP_SETUP.md#installation)
- CLI Usage: [CLI_USAGE.md](./CLI_USAGE.md)
- Troubleshooting: [SERENA_MCP_SETUP.md § Troubleshooting](./SERENA_MCP_SETUP.md#troubleshooting)
- Performance: [SERENA_MCP_SETUP.md § Performance Optimization](./SERENA_MCP_SETUP.md#performance-optimization)

### Security

- Read-Only Mode: [SERENA_MCP_SETUP.md § Security Considerations](./SERENA_MCP_SETUP.md#security-considerations)
- Tool Restrictions: [CLI_USAGE.md § Project Configuration](./CLI_USAGE.md#projectyml-structure)

---

## 📝 Cheat Sheets

### Quick Setup (Windows)

```powershell
# 1. Install Serena
cd T:\projects\serena-source
uv pip install -e .

# 2. Create config
uv run serena config edit

# 3. Configure MCP (already done - see MCP_CONFIG_CHANGES.md)
# Edit C:\Users\david\mcp.json

# 4. Setup project
cd C:\your\project
uv run serena project generate-yml
uv run serena project index

# 5. Restart Claude Desktop
```

### Common CLI Commands

```bash
# Server
serena start-mcp-server --project /path/to/project

# Project
serena project generate-yml
serena project index
serena project health-check

# Modes
serena mode list
serena mode create --name my-mode
serena mode edit my-mode

# Tools
serena tools list
serena tools description find_symbol

# Config
serena config edit
```

### Common Issues & Fixes

| Problem | Solution |
|---------|----------|
| "uvx not found" | Add `C:\Users\david\bin` to PATH |
| "No symbols found" | Run `serena project index` |
| "Language server failed" | Run `serena project health-check` |
| "Tool timeout" | Increase `tool_timeout` in config |
| "Project not found" | Register in `serena_config.yml` |

---

## 🔗 External Resources

- **GitHub**: https://github.com/oraios/serena
- **Issues**: https://github.com/oraios/serena/issues
- **MCP Protocol**: https://modelcontextprotocol.io/
- **uv Documentation**: https://docs.astral.sh/uv/

---

## 📍 File Locations

### User Files (Windows)

- Global Config: `C:\Users\david\.serena\serena_config.yml`
- MCP Config: `C:\Users\david\mcp.json`
- Logs: `C:\Users\david\.serena\logs\`
- Custom Modes: `C:\Users\david\.serena\modes\`
- Custom Contexts: `C:\Users\david\.serena\contexts\`
- Prompt Overrides: `C:\Users\david\.serena\prompt_templates\`

### Project Files

- Project Config: `<project>/.serena/project.yml`
- LSP Cache: `<project>/.serena/lsp-cache/`
- Memories: `<project>/.serena/memories/`
- Health Check Logs: `<project>/.serena/logs/health-checks/`

### Source Files

- Serena Source: `T:\projects\serena-source\`
- Templates: `T:\projects\serena-source\src\serena\resources\`
- Documentation: `T:\projects\serena-source\*.md`

---

## 🆘 Getting Help

### Troubleshooting Resources

1. **Check logs first**: `~/.serena/logs/mcp_*.log`
2. **Run health check**: `serena project health-check`
3. **Review troubleshooting guide**: [SERENA_MCP_SETUP.md § Troubleshooting](./SERENA_MCP_SETUP.md#troubleshooting)
4. **Check GitHub issues**: https://github.com/oraios/serena/issues

### Support Channels

- **GitHub Issues**: Bug reports and feature requests
- **Documentation**: This index and linked guides
- **Logs**: Check `~/.serena/logs/` for diagnostics
- **Dashboard**: http://localhost:24282/dashboard/ (if enabled)

---

## 🚀 What's Next?

### Planned Documentation

- [ ] Python Library API Guide
- [ ] Dashboard API Reference
- [ ] Advanced Tool Development Patterns
- [ ] CI/CD Integration Examples
- [ ] Multi-Language Project Setup
- [ ] Performance Tuning Guide
- [ ] Security Best Practices

### Contributing to Documentation

Documentation contributions are welcome! See the main README for contribution guidelines.

---

**Documentation Maintained By**: Serena Development Team  
**Last Major Update**: January 10, 2025  
**Documentation Version**: 1.0
