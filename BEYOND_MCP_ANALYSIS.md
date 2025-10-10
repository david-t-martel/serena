#### MCP Server as Daemon

**Command**: `serena start-mcp-server`

**Purpose**: Long-running MCP server process for integration with LLM clients

**Configuration Options**:

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

**Usage Examples**:

```bash
# Basic server startup
serena start-mcp-server --project /path/to/project

# Development mode with debugging
serena start-mcp-server \
  --project ./my-project \
  --log-level DEBUG \
  --trace-lsp-communication true \
  --enable-web-dashboard true

# Production mode, read-only context
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

**Server Lifecycle**:
1. **Initialization**: Load configuration, create SerenaAgent
2. **Tool Registration**: Register all exposed tools via MCP protocol
3. **Transport Binding**: Bind to stdio/HTTP/SSE transport
4. **Event Loop**: Listen for tool call requests
5. **Execution**: Process requests sequentially via SerenaAgent
6. **Shutdown**: Graceful cleanup of language servers and resources

**Evidence**: `src/serena/cli.py` lines 106-192, `CLI_USAGE.md` lines 77-134, `src/serena/mcp.py` lines 246-300

#### Project Management Commands

**Command Group**: `serena project`

**Available Commands**:

1. **Generate Project Configuration**
```bash
serena project generate-yml [PROJECT_PATH]
# Creates .serena/project.yml with default settings
```

2. **Index Project Symbols**
```bash
serena project index [PROJECT_PATH]
# Pre-indexes all symbols for faster tool execution
```

3. **Index Single File**
```bash
serena project index-file FILE_PATH [PROJECT_PATH]
# Indexes symbols in a specific file
```

4. **Health Check**
```bash
serena project health-check [PROJECT_PATH]
# Validates LSP integration and configuration
```

**Evidence**: `CLI_USAGE.md` (project management sections), `src/serena/cli.py` project command group

#### Mode and Context Management

**Mode Commands** (`serena mode`):
```bash
# List all available modes
serena mode list

# Create new mode from template
serena mode create --name my-custom-mode

# Copy and customize an internal mode
serena mode create --from-internal editing --name my-editing

# Edit existing custom mode
serena mode edit my-custom-mode

# Delete custom mode
serena mode delete my-custom-mode
```

**Context Commands** (`serena context`):
```bash
# List all available contexts
serena context list

# Create new context
serena context create --name my-context

# Copy and customize internal context
serena context create --from-internal agent --name my-agent

# Edit existing context
serena context edit my-context

# Delete custom context
serena context delete my-context
```

**Configuration Commands** (`serena config`):
```bash
# Edit global configuration
serena config edit
# Opens ~/.serena/serena_config.yml in default editor
```

**Evidence**: `src/serena/cli.py` lines 239-311 (modes), 314-388 (contexts), 391-410 (config), `CLI_USAGE.md` lines 172-232

#### Tool Introspection Commands

**Command Group**: `serena tools`

```bash
# List all available tools
serena tools list [--project PROJECT]

# Get tool description
serena tools description TOOL_NAME [--project PROJECT]
# Shows: Tool name, description, parameters, return type
```

**Use Cases**:
- Discover what tools are available
- Understand tool parameters before using
- Validate tool availability in specific modes/contexts

**Evidence**: `CLI_USAGE.md` (tools section), `src/serena/cli.py` tools command group

#### System Prompt Generation

**Command**: `serena print-system-prompt`

**Purpose**: Generate and display the system prompt that would be used for a project.

**Usage**:
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

**Output**: Complete system prompt including:
- Context-specific instructions
- Mode-specific instructions
- Tool availability and descriptions
- Project-specific initial prompt

**Use Cases**:
- Preview what instructions LLM will receive
- Debug unexpected tool behavior
- Customize prompts via context/mode overrides

**Evidence**: `src/serena/cli.py` lines 195-236, `CLI_USAGE.md` lines 137-169

#### Daemon Characteristics

When running as MCP server, Serena operates as a **long-running daemon** with:

**Resource Management**:
- Language server process lifecycle management
- Automatic restart on termination errors
- Cache persistence across restarts
- Memory-efficient symbol indexing

**Logging**:
- Log rotation and management
- Multiple log handlers (file, stderr, memory)
- Configurable log levels per component
- Trace-level LSP communication logging

**Monitoring** (optional):
- Web dashboard on port 24282
- Real-time log streaming
- Tool execution history
- Performance metrics

**Robustness**:
- Graceful error handling
- Automatic LSP server restarts
- Tool timeout protection
- Safe shutdown on signals

**Evidence**: `src/serena/cli.py` lines 155-192 (logging setup), `src/serena/mcp.py` lines 302-349 (server lifespan)

---

### 8.5 Mode 4: MCP Server (Context Setting)

**Covered in Part 1** - MCP Server Baseline

The MCP Server mode is the **standard integration** for LLM clients like Claude Desktop. It provides:

- Tool registration via MCP protocol
- JSON-RPC based tool calling
- stdio/HTTP/SSE transports
- Stateful conversation management
- Real-time execution with streaming

**Key Distinction**: MCP Server is a **communication protocol wrapper** around the Python Library API. All underlying capabilities (LSP integration, tool execution, project management) remain the same across all modes.

**Evidence**: Part 1 of this document, `src/serena/mcp.py`

---

### 8.6 Integration Mode Comparison

| Aspect | Python Library | Custom Agents | CLI/Daemon | MCP Server |
|--------|---------------|---------------|------------|------------|
| **Use Case** | Scripts, automation | Framework integration | Standalone, CI/CD | LLM clients |
| **Entry Point** | `SerenaAgent` class | Wrapped tools | `serena` command | `serena start-mcp-server` |
| **Protocol** | Direct Python calls | Framework-specific | CLI arguments | MCP (JSON-RPC) |
| **Flexibility** | ✅ Highest | ✅ High | ⚠️ Moderate | ⚠️ Protocol-bound |
| **Ease of Use** | ⚠️ Requires coding | ⚠️ Requires integration | ✅ Simple CLI | ✅ Standard protocol |
| **Performance** | ✅ Best (no overhead) | ✅ Good | ✅ Good | ⚠️ Protocol overhead |
| **Async Support** | ✅ Via `issue_task` | ✅ Framework-dependent | ❌ Sequential | ✅ Via MCP streaming |
| **Tool Discovery** | ✅ Programmatic | ✅ Programmatic | ✅ Via `tools list` | ✅ Via MCP protocol |
| **Configuration** | ✅ Code-based | ✅ Code-based | ✅ CLI flags/files | ✅ CLI flags/files |
| **Logging** | ✅ Configurable | ✅ Configurable | ✅ File + stderr | ✅ File + stderr + memory |
| **Dashboard** | ✅ Optional | ✅ Optional | ✅ Optional | ✅ Optional |
| **Best For** | Custom automation | AI agent frameworks | CLI workflows | Claude Desktop |

---

### 8.7 Key Insights

#### Unified Core, Multiple Interfaces

All four integration modes share the **same underlying implementation**:
- LSP-powered semantic engine
- Tool execution framework
- Project management system
- Configuration system (contexts/modes)

The modes differ **only in the interface layer**:
- Python Library: Direct API access
- Custom Agents: Wrapped API access
- CLI: Command-line interface
- MCP Server: Protocol wrapper

#### Composability

Modes can be **combined** in workflows:

**Example 1: CLI Pre-Indexing + MCP Server**
```bash
# Pre-index project offline
serena project index /path/to/project

# Start MCP server (uses pre-built cache)
serena start-mcp-server --project /path/to/project
```

**Example 2: Python Library + Custom Agent**
```python
# Use Python Library to create custom agent
agent = SerenaAgent(project="/path")
custom_agent = CustomFramework(tools=wrap_serena_tools(agent))
```

**Example 3: CLI for Setup, MCP for Operation**
```bash
# Generate project configuration
serena project generate-yml /path/to/project

# Edit configuration
serena config edit

# Start MCP server with custom config
serena start-mcp-server --project /path/to/project
```

#### No Lock-In

Serena's multi-mode design **avoids vendor lock-in**:
- Not dependent on any single LLM provider
- Not bound to MCP protocol exclusively
- Can be used standalone without AI
- Embeddable in any Python application

**Evidence**: Multi-mode architecture described throughout `PYTHON_LIBRARY_API.md`, `CLI_USAGE.md`, and source code

---

**Document Status**: Integration Modes Analysis Complete  
**Next Analysis Phase**: Complete Tool Inventory  
**Estimated Completion**: 85% of overall analysis
