# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Essential Development Commands

**ALWAYS use these exact commands for development tasks:**

```bash
# Format code (BLACK + RUFF) - ONLY allowed formatting command
uv run poe format

# Run mypy type checking - ONLY allowed type checking command
uv run poe type-check

# Run tests with default markers (excludes java/rust/erlang by default)
uv run poe test

# Run specific language tests
uv run poe test -m "python or go"
uv run poe test -m "snapshot"

# Check code style without fixing
uv run poe lint

# Start MCP server from project root
uv run serena-mcp-server

# Index project for faster tool performance
uv run index-project
```

**Available pytest markers for selective testing:**
- Language markers: `python`, `go`, `java`, `rust`, `typescript`, `php`, `csharp`, `elixir`, `terraform`, `clojure`, `swift`, `bash`, `ruby`, `ruby_solargraph`, `zig`, `lua`, `nix`, `dart`, `erlang`, `al`
- Feature markers: `snapshot` (for symbolic editing operation tests)

**Always run format, type-check, and test before completing any task.**

## High-Level Architecture

Serena is a dual-layer coding agent toolkit that provides IDE-like semantic code tools to LLMs.

### Core Components

**1. SerenaAgent (`src/serena/agent.py`)**
- Central orchestrator managing projects, tools, and user interactions
- Coordinates language servers, memory persistence, and MCP server interface
- Manages tool registry and context/mode configurations
- Handles project activation, memories, and configuration hierarchy

**2. SolidLanguageServer (`src/solidlsp/ls.py`)**
- Unified wrapper around Language Server Protocol (LSP) implementations
- Provides language-agnostic interface for symbol operations
- Handles caching, error recovery, and multiple language server lifecycle
- Enables semantic understanding of code across 16+ languages

**3. Tool System (`src/serena/tools/`)**
```
tools/
├── file_tools.py      # File system operations, search, regex replacements
├── symbol_tools.py    # Language-aware symbol finding, navigation, editing
├── memory_tools.py    # Project knowledge persistence and retrieval
├── config_tools.py    # Project activation, mode switching
└── workflow_tools.py  # Onboarding and meta-operations
```

**4. Configuration System (`src/serena/config/`)**
- **Contexts** - Define tool sets for different environments (desktop-app, agent, ide-assistant, codex)
- **Modes** - Operational patterns (planning, editing, interactive, one-shot, no-onboarding)
- **Projects** - Per-project settings (`.serena/project.yml`) and language server configs

### Symbol-Based Editing Philosophy

Serena uses **LSP-based semantic understanding** rather than text-based analysis:
- Tools operate on **symbols** (classes, functions, methods) rather than line numbers
- Edits are **structurally aware** - know about classes, methods, imports
- Cross-file references are tracked through LSP's "find references" capability
- Caching reduces language server overhead while maintaining accuracy

This approach excels in large, strongly-structured codebases where precise code navigation is crucial.

## Language Support Architecture

Each supported language has:

1. **Language Server Implementation** in `src/solidlsp/language_servers/`
   - Subclasses `SolidLanguageServer`
   - Implements language-specific initialization and communication
   - May include automatic download/installation logic

2. **Language Registration** in `src/solidlsp/ls_config.py`
   - Added to `Language` enum
   - File extensions mapped via `get_source_fn_matcher()`

3. **Test Repository** in `test/resources/repos/<language>/test_repo/`
   - Minimal project with realistic symbol structures
   - Classes, functions, imports, nested structures
   - Used for integration testing

4. **Test Suite** in `test/solidlsp/<language>/`
   - Tests for symbol finding, reference finding, cross-file operations
   - Uses pytest markers for selective execution
   - Must verify actual symbol names/references, not just non-null results

**Example languages:**
- Python (pyright) - batteries included, no external dependencies
- Go (gopls) - requires `gopls` installation
- TypeScript/JavaScript - automatic language server download
- PHP (Intelephense) - automatic download, premium features via `INTELEPHENSE_LICENSE_KEY`
- Rust (rust-analyzer) - uses toolchain via rustup

## Development Patterns

### Adding New Languages

Follow `.serena/memories/adding_new_language_support_guide.md`:

1. Create language server class in `src/solidlsp/language_servers/<language>_server.py`
2. Add to `Language` enum in `src/solidlsp/ls_config.py`
3. Update factory method in `src/solidlsp/ls.py` 
4. Create test repository in `test/resources/repos/<language>/test_repo/`
5. Write test suite in `test/solidlsp/<language>/test_<language>_basic.py`
6. Add pytest marker to `pyproject.toml`
7. Update README.md with new language support

### Adding New Tools

1. Inherit from `Tool` base class in `src/serena/tools/tools_base.py`
2. Implement `apply()` method with proper type annotations
3. Add docstring with tool description (becomes MCP tool description)
4. Register in appropriate tool registry (file_tools, symbol_tools, etc.)
5. Add to context/mode configurations as needed
6. Consider optional tools vs. default tools

### Testing Strategy

- **Language-specific tests** use pytest markers for selective execution
- **Snapshot tests** (`-m snapshot`) verify symbolic editing operations
- **Integration tests** in `test/serena/test_serena_agent.py` test full workflows
- **Test repositories** provide realistic symbol structures for testing
- All tests must verify actual results (symbol names, file paths) not just non-null

### Running Tools Locally (Without LLM)

You can test Serena's tools directly without an MCP client or LLM. See `scripts/demo_run_tools.py`:

```python
from serena.agent import SerenaAgent
from serena.config.serena_config import SerenaConfig
from serena.tools import FindSymbolTool, FindReferencingSymbolsTool

agent = SerenaAgent(
    project="/path/to/project",
    serena_config=SerenaConfig(gui_log_window_enabled=False, web_dashboard=False)
)

find_symbol_tool = agent.get_tool(FindSymbolTool)
result = agent.execute_task(lambda: find_symbol_tool.apply("MyClass"))
```

## Configuration Hierarchy

Configuration is loaded from (in order of precedence):

1. **Command-line arguments** to `serena-mcp-server` (highest priority)
2. **Project-specific config** `.serena/project.yml` in project directory
3. **User config** `~/.serena/serena_config.yml`
4. **Active context** (desktop-app, agent, ide-assistant, codex)
5. **Active modes** (planning, editing, interactive, one-shot, no-onboarding)

### Context vs. Modes

- **Context**: Set at startup, defines environment (Claude Desktop vs. IDE vs. terminal CLI)
  - Controls which tools are available by default
  - Adjusts prompts for client capabilities
  - Cannot be changed during session
  
- **Modes**: Can be switched mid-session via `switch_modes` tool
  - Refines behavior for specific task types
  - Multiple modes can be active simultaneously
  - Examples: planning+one-shot for reports, editing+interactive for iterative development

## Key Implementation Notes

### Memory System

- Markdown files in `.serena/memories/` directory
- **Onboarding** process creates initial memories on first project activation
- Memories are **loaded on demand** by the agent (not always in context)
- Used to persist project knowledge across sessions
- Enables conversation continuity without context bloat

### Project Indexing

For large projects, **index before first use** to accelerate tool performance:

```bash
uv run serena project index
# or from any directory:
uv run serena project index /path/to/project
```

Indexing creates cached metadata about project structure, reducing initial tool latency.

### Language Servers

- Run as **separate processes** communicating via LSP
- Automatically started/stopped by `SolidLanguageServer`
- Support **automatic recovery** on crashes
- Some languages auto-download their language servers on first use
- Cache symbol lookups to reduce LSP overhead

### MCP Protocol Integration

- Serena exposes tools to AI agents via Model Context Protocol (MCP)
- Tools are automatically converted to MCP tool definitions
- Parameter validation via Pydantic models
- Supports both stdio and Streamable HTTP transports

### Dashboard and Logging

- Web dashboard runs on `http://localhost:24282/dashboard/index.html` (or next available port)
- Shows tool usage stats if `record_tool_usage_stats: true` in config
- Provides shutdown capability (some MCP clients don't cleanup processes properly)
- GUI log viewer available on Windows/Linux (not macOS)

## Important Notes for Development

### Project Structure

- Python 3.11 (not 3.12+) with strict typing
- Managed by `uv` - do not use pip/poetry/conda
- Tasks via `poe` (poethepoet) - defined in `pyproject.toml`
- Black line length: 140 characters
- No relative imports (TID252 disabled)

### Git Configuration on Windows

**CRITICAL:** Set `git config core.autocrlf true` globally on Windows:

```bash
git config --global core.autocrlf true
```

Without this, line ending changes will appear as massive diffs when Serena writes files.

### Code Style

- Type annotations required on all functions (`disallow_untyped_defs`)
- Mypy strict mode enabled
- Use `override` decorator for overridden methods
- Ruff with comprehensive rule set (see `pyproject.toml`)
- Many D-rules (docstring) disabled for pragmatism

### Testing

- Default test run excludes slow tests: `not java and not rust and not erlang`
- Override with custom markers: `PYTEST_MARKERS="python or typescript" uv run poe test`
- Or use CLI: `uv run poe test -m "python or go"`
- Snapshot tests use `syrupy` library

### Environment Setup

**Using uv (recommended):**

```bash
uv venv
source .venv/bin/activate  # Linux/macOS/Git Bash
# or
.venv\Scripts\activate     # Windows cmd/PowerShell

uv pip install --all-extras -r pyproject.toml -e .
```

### Docker Support (Experimental)

Available via `compose.yaml` but has limitations (see `DOCKER.md`):
- Better security isolation
- No need for local language server installations
- Mount project directories as volumes
- Dashboard on port 24282, MCP server on 9121

## Customization and Extension

### Creating Custom Contexts/Modes

```bash
# List available contexts/modes
uv run serena context --help
uv run serena mode --help

# Custom contexts/modes are YAML files in ~/.serena/
# Automatically registered by filename (without .yml extension)
```

### Prompt Customization

- Tool descriptions come from docstrings
- System prompts defined in `src/interprompt/` using Jinja2 templates
- Contexts and modes can override prompts and tool descriptions
- See `src/serena/prompt_factory.py` for prompt generation

### Extending the Tool System

Tools automatically integrate if they:
1. Subclass `Tool` from `src/serena/tools/tools_base.py`
2. Implement `apply()` with proper parameter types
3. Are imported in the tool registry

No manual registration needed - `SerenaAgent` discovers tools automatically.

## Example Workflows

### Starting Development Session

```bash
# Activate virtual environment
source .venv/bin/activate

# Pull latest changes
git pull

# Run checks
uv run poe format
uv run poe type-check
uv run poe lint

# Run tests for your language
uv run poe test -m "python"

# Start MCP server for testing
uv run serena-mcp-server --context ide-assistant
```

### Adding a New Tool

```python
# src/serena/tools/my_new_tool.py
from serena.tools.tools_base import Tool

class MyNewTool(Tool):
    """Brief description of what this tool does."""
    
    def apply(self, param: str) -> str:
        """
        Detailed description.
        
        :param param: parameter description
        :return: result description
        """
        # Implementation
        return result
```

### Debugging Language Server Issues

```python
# Enable detailed logging
agent = SerenaAgent(
    project="/path/to/project",
    serena_config=SerenaConfig(log_level="DEBUG")
)

# Check language server status
agent.language_server.is_running()

# Restart language server if needed (via tool)
restart_tool = agent.get_tool(RestartLanguageServerTool)
restart_tool.apply()
```

---

**Note:** The web dashboard will display at `http://localhost:24282/dashboard/index.html` (or the next available port if 24282 is taken). Use it to monitor tool usage and shut down the server cleanly.
