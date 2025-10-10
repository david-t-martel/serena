# Part 8: Integration Modes - Beyond MCP Server

## Overview

Serena provides **four primary integration modes**, each suited for different use cases:

1. **Python Library API** - Direct programmatic access for custom scripts and applications
2. **Custom Agent Frameworks** - Embedding Serena in bespoke AI systems (e.g., Agno, LangChain)  
3. **CLI / Daemon Mode** - Command-line operation and long-running background processes
4. **MCP Server** - Standard protocol for LLM clients like Claude Desktop

This section catalogs how to use modes 1-3, complementing the MCP baseline established in Part 1.

**Evidence**: `PYTHON_LIBRARY_API.md`, `CLI_USAGE.md`, `src/serena/cli.py`, `src/serena/mcp.py`

---

## 8.2 Mode 1: Python Library API

### Core Entry Point: SerenaAgent

**Definition**: Direct programmatic access to Serena's capabilities without protocol overhead.

**Evidence**: `src/serena/agent.py` lines 46-166, `PYTHON_LIBRARY_API.md` lines 79-103

### Tool Execution Patterns

- Pattern 1: Direct Tool Invocation via `agent.execute_task()`
- Pattern 2: Asynchronous Execution via `agent.issue_task()`
- Pattern 3: Direct Apply with `tool.apply_ex()`

### Thread-Safe Task Executor  

SerenaAgent ensures **linear, sequential execution** even with concurrent task submission, preventing race conditions.

### Project Management API

Methods for loading, activating, and switching between projects programmatically.

### Complete API Surface

Full API documented in PYTHON_LIBRARY_API.md including initialization, tool access, task execution, project management, configuration, and language server methods.

---

## 8.3 Mode 2: Custom Agent Framework Integration

### Agno Integration (Reference Implementation)

SerenaAgnoToolkit wraps Serena tools as Agno functions, demonstrating the integration pattern.

**Evidence**: `src/serena/agno.py`

### LangChain & AutoGPT Integration (Conceptual)

Patterns for converting Serena tools to framework-specific tool representations.

---

## 8.4 Mode 3: CLI and Daemon Operation  

### CLI Architecture

Entry point: `serena` command with command groups for server, modes, contexts, projects, config, tools, and prompts.

### MCP Server as Daemon

Long-running process with resource management, logging, monitoring, and robustness features.

### Project Management Commands

- `serena project generate-yml`
- `serena project index`
- `serena project index-file`
- `serena project health-check`

### Mode and Context Management

CLI commands for listing, creating, editing, and deleting custom modes and contexts.

### Tool Introspection Commands

`serena tools list` and `serena tools description` for discovering available tools.

### System Prompt Generation

`serena print-system-prompt` to preview instructions sent to LLM.

---

## 8.5 Mode 4: MCP Server (Context Setting)

Covered in Part 1 - MCP Server is a protocol wrapper around the Python Library API.

---

## 8.6 Integration Mode Comparison

| Aspect | Python Library | Custom Agents | CLI/Daemon | MCP Server |
|--------|---------------|---------------|------------|------------|
| **Use Case** | Scripts, automation | Framework integration | Standalone, CI/CD | LLM clients |
| **Entry Point** | `SerenaAgent` class | Wrapped tools | `serena` command | `serena start-mcp-server` |
| **Protocol** | Direct Python calls | Framework-specific | CLI arguments | MCP (JSON-RPC) |
| **Flexibility** | ✅ Highest | ✅ High | ⚠️ Moderate | ⚠️ Protocol-bound |
| **Best For** | Custom automation | AI agent frameworks | CLI workflows | Claude Desktop |

---

## 8.7 Key Insights

### Unified Core, Multiple Interfaces

All modes share the same underlying LSP-powered semantic engine, tool execution framework, project management, and configuration system.

### Composability  

Modes can be combined in workflows (e.g., CLI pre-indexing + MCP server operation).

### No Lock-In

Multi-mode design avoids vendor lock-in - not dependent on any single LLM provider or protocol.

---

**Document Status**: Integration Modes Analysis Complete  
**Next Analysis Phase**: Complete Tool Inventory  
**Estimated Completion**: 85% of overall analysis
