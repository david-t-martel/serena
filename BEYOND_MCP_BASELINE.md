# Serena: Beyond MCP Server - Baseline Established

This document tracks the progress of establishing what Serena offers **beyond the MCP (Model Context Protocol) server interface**.

## Status: ✅ Phase 2 Complete - Custom Tool Development

**Date**: January 10, 2025  
**Completed**: Steps 1-2 of the "Beyond MCP Server Baseline" investigation

---

## What We've Discovered

### 1. Python Library API Usage ✅ COMPLETE

**Documentation Created**: [`PYTHON_LIBRARY_API.md`](./PYTHON_LIBRARY_API.md)

Serena provides a **rich Python API** for programmatic usage:

#### Core Capabilities

- **Direct Tool Execution**: Execute any of Serena's 20+ tools programmatically
- **Agent Integration**: Integrate with LangChain, AutoGPT, Agno, and custom frameworks
- **Project Management**: Load, activate, and switch between projects
- **Task Execution**: Thread-safe, sequential task execution with `issue_task()` and `execute_task()`
- **Configuration**: Full control over tool sets, modes, contexts, and settings

#### Key API Components

**SerenaAgent Class**:
```python
from serena.agent import SerenaAgent
from serena.config.serena_config import SerenaConfig

agent = SerenaAgent(
    project="/path/to/project",
    serena_config=SerenaConfig(
        gui_log_window_enabled=False,
        web_dashboard=False
    )
)
```

**Tool Access**:
```python
from serena.tools import FindSymbolTool

tool = agent.get_tool(FindSymbolTool)
result = agent.execute_task(lambda: tool.apply(name_path="MyClass"))
```

**Project Management**:
```python
project = agent.activate_project_from_path_or_name("/path")
project = agent.get_active_project()
source_files = project.gather_source_files()
```

#### Practical Use Cases Documented

1. **Custom Automation Scripts**: Batch symbol analysis, code migration tools
2. **Agent Framework Integration**: Wrapping Serena tools for Agno, LangChain, AutoGPT
3. **Batch Processing**: Indexing, documentation generation, multi-file operations
4. **Custom Workflows**: Combining multiple tools programmatically
5. **Embedded Applications**: Using Serena as a library in larger systems

#### Integration Examples

**Agno Integration** (from `src/serena/agno.py`):
- Wraps Serena tools as Agno functions
- Converts tool docstrings to function metadata
- Provides seamless integration with Agno's agent system

**LangChain Integration Pattern**:
```python
from langchain.tools import Tool as LangChainTool

langchain_tools = [
    LangChainTool(
        name=tool.get_name_from_cls(),
        description=tool.get_apply_docstring(),
        func=lambda **kw: tool.apply_ex(**kw)
    )
    for tool in agent.get_exposed_tool_instances()
]
```

---

## Remaining Investigation Areas

### 2. Custom Tool Development ✅ COMPLETE

**Documentation Created**: [`CUSTOM_TOOLS.md`](./CUSTOM_TOOLS.md)

Serena provides a **powerful framework for creating custom tools**:

#### Core Concepts Documented

- **Tool Architecture**: Base `Tool` class, automatic registration, lifecycle
- **Tool Markers**: Five marker types for controlling tool behavior and availability
- **Agent Resources**: Accessing projects, language servers, code editors, memories, etc.
- **Registration**: Automatic discovery via reflection

#### Tool Markers Explained

1. **`ToolMarkerCanEdit`**: Marks editing tools, disabled in read-only mode
2. **`ToolMarkerSymbolicRead`**: Marks semantic read operations via LSP
3. **`ToolMarkerSymbolicEdit`**: Marks semantic edit operations
4. **`ToolMarkerOptional`**: Marks tools disabled by default
5. **`ToolMarkerDoesNotRequireActiveProject`**: Tool works without active project

#### Accessing Resources from Tools

**Project Access**:
```python
self.project.read_file("src/main.py")
self.project.gather_source_files()
self.project.is_ignored_path(path)
```

**Language Server Access**:
```python
symbol_retriever = self.create_language_server_symbol_retriever()
symbols = symbol_retriever.find_by_name("MyClass")
```

**Code Editor Access**:
```python
code_editor = self.create_code_editor()
code_editor.replace_body(name_path, relative_file_path, body)
```

**Memory Manager**:
```python
self.memories_manager.save_memory(name, content)
content = self.memories_manager.load_memory(name)
```

#### Advanced Patterns

1. **File Editing with Context Manager**: `EditedFileContext` for safe edits
2. **Multi-Step Operations**: Combining retrieval and analysis
3. **Tool Composition**: Using existing tools together
4. **Configurable Tools**: Multiple modes and parameters
5. **Error Handling**: Robust error messages
6. **Progressive Results**: Early exit and streaming

#### Complete Examples Provided

1. **FileStatsTool**: Calculate file statistics (lines, size, by extension)
2. **ComplexityAnalysisTool**: Find high-complexity classes/functions
3. **DependencyMapperTool**: Map import dependencies between files
4. **TestCoverageTool**: Analyze test coverage by file matching

#### Best Practices Documented

- Docstring guidelines for LLMs
- Type annotations
- Return value formats (JSON preferred)
- Error message clarity
- Performance considerations
- Project path validation
- Language server availability checks

---

**Goal**: Document how to create custom tools that extend Serena's capabilities.

**Key Questions**:
- How to inherit from `Tool` base class?
- Tool markers: `CanEdit`, `SymbolicRead`, `SymbolicEdit`, `Optional`, `DoesNotRequireActiveProject`
- How are tools registered automatically?
- How to write proper `apply()` method docstrings for LLM consumption?
- How to access project, language server, and memories from custom tools?

**Files to Review**:
- `src/serena/tools/tools_base.py` (Tool class, ToolRegistry)
- Existing tool implementations in `src/serena/tools/`

---

### 3. CLI and Daemon Modes 🔜 NEXT

**Goal**: Document non-MCP ways to run Serena.

**Key Questions**:
- What CLI commands are available beyond `start-mcp-server`?
- Is there a daemon mode?
- REPL or interactive CLI?
- How to use `serena` CLI for project management, indexing, health checks?

**Files to Review**:
- `src/serena/cli.py` (comprehensive CLI with many commands)
- Command groups: `mode`, `context`, `project`, `config`, `tools`, `prompts`

---

### 4. Dashboard and GUI Components 🔜 PENDING

**Goal**: Document UI/monitoring capabilities.

**Key Questions**:
- How does the web dashboard work?
- Can it be embedded in other apps?
- GUI log viewer functionality?
- How to enable/disable these features programmatically?

**Implementation Details**:
- `agent._gui_log_viewer`: GUI log window (tkinter-based)
- `agent._dashboard_thread`: Web dashboard API
- Config flags: `web_dashboard`, `gui_log_window_enabled`

---

### 5. Advanced Configuration 🔜 PENDING

**Goal**: Document all configuration options.

**Key Topics**:
- `SerenaConfig` class: All options and defaults
- `ProjectConfig` class: Project-specific settings
- Tool sets and inclusion definitions
- Mode switching and mode configuration
- Context system and custom contexts

**Files to Review**:
- `src/serena/config/serena_config.py`
- Context and mode YAML files

---

### 6. Language Server Customization 🔜 PENDING

**Goal**: Document direct language server API usage.

**Key Topics**:
- `SolidLanguageServer` API (49 methods)
- Creating custom language server adapters
- 26 pre-configured language servers (Python, TypeScript, Go, Rust, etc.)
- Caching, request timeout, cross-file referencing
- Direct LSP operations without going through tools

**Files to Review**:
- `src/solidlsp/ls.py`
- `src/solidlsp/language_servers/*.py`

---

### 7. Symbol and Code Analysis 🔜 PENDING

**Goal**: Document semantic code analysis capabilities.

**Key Topics**:
- `LanguageServerSymbolRetriever` direct usage
- Symbol finding patterns and name path matching
- Depth parameter for symbol hierarchies
- Finding references, definitions, containers
- Direct symbol tree navigation

**Files to Review**:
- `src/serena/symbol.py`
- `src/serena/tools/symbol_tools.py`

---

### 8. Memory System 🔜 PENDING

**Goal**: Document project-specific knowledge storage.

**Key Topics**:
- `MemoriesManager` API
- Memory file format and location
- Reading/writing memories programmatically
- Memory lifecycle and management

**Implementation**:
- Memories stored in `.serena/memories/` directory
- Markdown format for human readability
- Project-specific storage

---

## Key Findings from Analysis

### Tool Catalog (20+ Tools Found)

**File Operations**:
- `ReadFileTool`, `CreateTextFileTool`, `ListDirTool`, `FindFileTool`
- `ReplaceRegexTool`, `DeleteLinesTool`, `ReplaceLinesTool`, `InsertAtLineTool`
- `SearchForPatternTool`

**Symbol Operations**:
- `FindSymbolTool`, `FindReferencingSymbolsTool`, `GetSymbolsOverviewTool`
- `ReplaceSymbolBodyTool`, `InsertAfterSymbolTool`, `InsertBeforeSymbolTool`

**JetBrains Integration**:
- `JetBrainsFindSymbolTool`, `JetBrainsFindReferencingSymbolsTool`
- `JetBrainsGetSymbolsOverviewTool`

**Memory Management**:
- `WriteMemoryTool`, `ReadMemoryTool`, `ListMemoriesTool`, `DeleteMemoryTool`

**Configuration**:
- `ActivateProjectTool`, `RemoveProjectTool`, `SwitchModesTool`, `GetCurrentConfigTool`

**Workflow Tools**:
- `OnboardingTool`, `ThinkAboutCollectedInformationTool`
- `ThinkAboutTaskAdherenceTool`, `ThinkAboutWhetherYouAreDoneTool`
- `SummarizeChangesTool`, `PrepareForNewConversationTool`

**Command Execution**:
- `ExecuteShellCommandTool`

### Language Server Support (26 Languages)

- Python (Pyright, Jedi)
- TypeScript/JavaScript
- Go (gopls)
- Rust (rust-analyzer)
- Java (Eclipse JDT.LS)
- C/C++ (clangd)
- C# (OmniSharp, C# LSP)
- Ruby (Ruby LSP, Solargraph)
- Kotlin
- Swift (SourceKit-LSP)
- PHP (Intelephense)
- Lua (lua-ls)
- Bash
- Clojure
- Dart
- Erlang
- R
- Terraform
- Nix (nixd)
- Zig (zls)
- VimScript
- AL (Business Central)

### Configuration System

**Classes Identified**:
- `SerenaConfig`: Global configuration
- `ProjectConfig`: Project-specific settings
- `RegisteredProject`: Project registration
- `SerenaPaths`: Path management
- `ToolSet`: Tool inclusion/exclusion
- `ToolInclusionDefinition`: Tool filtering interface
- `SerenaConfigError`: Configuration errors

### MCP Integration

**Classes Identified**:
- `SerenaMCPFactory`: Main MCP server factory
- `SerenaMCPFactorySingleProcess`: Single-process variant
- `SerenaMCPRequestContext`: Request context management

---

## Example Scripts Available

1. **`scripts/demo_run_tools.py`**: Direct tool execution example
2. **`scripts/agno_agent.py`**: Agno framework integration
3. **`scripts/mcp_server.py`**: MCP server entry point
4. **`scripts/gen_prompt_factory.py`**: Prompt generation
5. **`scripts/print_tool_overview.py`**: Tool documentation
6. **`scripts/print_mode_context_options.py`**: Mode configuration

---

## Next Actions

1. ✅ **COMPLETED**: Document Python Library API usage
2. 🔜 **NEXT**: Document Custom Tool Development
3. 🔜 Document CLI commands and usage patterns
4. 🔜 Document Dashboard and GUI components
5. 🔜 Document Advanced Configuration options
6. 🔜 Document Language Server direct API
7. 🔜 Document Symbol Analysis capabilities
8. 🔜 Document Memory System

---

## Resources Created

- ✅ **`PYTHON_LIBRARY_API.md`**: Comprehensive Python API documentation with 10 examples
- ✅ **`CUSTOM_TOOLS.md`**: Complete guide to creating custom tools with 4 working examples
- ✅ **`serena_analysis_results.json`**: Detailed catalog of all components
- ✅ **`BEYOND_MCP_BASELINE.md`**: This tracking document

---

## Summary

Serena offers **far more than just an MCP server**. It's a complete Python library for semantic code analysis and manipulation, with:

- **Rich programmatic API** via `SerenaAgent`
- **20+ production-ready tools** for file and symbol operations
- **26 language servers** for multi-language support
- **Framework integration support** (Agno, LangChain, AutoGPT)
- **Custom tool development** capabilities
- **Comprehensive CLI** for project management
- **Web dashboard** and GUI log viewer
- **Flexible configuration** system
- **Memory management** for project knowledge

The Python Library API documentation provides a solid foundation for developers who want to use Serena programmatically, integrate it into their own tools, or build custom agents that leverage Serena's powerful semantic analysis capabilities.

---

**Last Updated**: January 10, 2025
