# Serena Python Library API - Beyond MCP Server

This document describes how to use Serena as a **Python library** in your own applications, scripts, and custom agents — going beyond the MCP (Model Context Protocol) server interface.

## Table of Contents

1. [Overview](#overview)
2. [Core Concepts](#core-concepts)
3. [Basic Usage Patterns](#basic-usage-patterns)
4. [Advanced Integration Patterns](#advanced-integration-patterns)
5. [Tool Execution](#tool-execution)
6. [Project Management](#project-management)
7. [Configuration](#configuration)
8. [Agent Framework Integration (Agno Example)](#agent-framework-integration-agno-example)
9. [API Reference](#api-reference)
10. [Examples](#examples)

---

## Overview

Serena provides a rich Python API that allows you to:

- **Programmatically execute tools** without going through MCP
- **Integrate with existing agent frameworks** (LangChain, AutoGPT, Agno, etc.)
- **Build custom workflows** and automation scripts
- **Embed Serena's capabilities** into larger applications
- **Create custom CLI tools** for your team's specific needs
- **Access language server features** directly in Python

The core of this API is the **`SerenaAgent`** class, which provides access to all tools, project management, and language server capabilities.

---

## Core Concepts

### SerenaAgent

The **`SerenaAgent`** is the central entry point for all Serena functionality. It manages:

- **Projects**: Loading, activating, and switching between projects
- **Tools**: Access to all symbolic and file-based tools
- **Language Server**: Integration with language servers for semantic analysis
- **Configuration**: Tool sets, modes, contexts, and project-specific settings
- **Task Execution**: Thread-safe, sequential execution of operations
- **Memories**: Project-specific knowledge storage

### Tools

Tools are Python classes that inherit from `Tool` and implement an `apply()` method. They provide:

- **Symbolic operations**: Find symbols, references, get symbol overviews
- **File operations**: Read, write, search, list files
- **Memory operations**: Store and retrieve project knowledge
- **Configuration operations**: Activate projects, switch modes
- **Workflow operations**: Onboarding, thinking tools, summarization

### Projects

Projects represent codebases with their own:

- Configuration (`project.yml`)
- Language server settings
- Tool inclusion/exclusion rules
- Memory storage
- Indexing cache

### Contexts and Modes

- **Context**: The operational environment (e.g., "agent", "ide_assistant")
- **Modes**: Modifiers that affect available tools and prompts (e.g., "editing", "planning", "interactive")

---

## Basic Usage Patterns

### Pattern 1: Simple Tool Execution

```python
from serena.agent import SerenaAgent
from serena.config.serena_config import SerenaConfig
from serena.tools import GetSymbolsOverviewTool, FindSymbolTool

# Create agent for a project
agent = SerenaAgent(
    project="/path/to/your/project",
    serena_config=SerenaConfig(
        gui_log_window_enabled=False,
        web_dashboard=False
    )
)

# Get a tool instance
overview_tool = agent.get_tool(GetSymbolsOverviewTool)

# Execute the tool synchronously
result = agent.execute_task(
    lambda: overview_tool.apply("src/myfile.py")
)

print(result)  # JSON string with symbol information
```

### Pattern 2: Working Without a Project

```python
from serena.agent import SerenaAgent
from serena.config.serena_config import SerenaConfig

# Create agent without activating a project
agent = SerenaAgent(
    project=None,
    serena_config=SerenaConfig(web_dashboard=False)
)

# Only tools marked with ToolMarkerDoesNotRequireActiveProject will work
# Examples: ActivateProjectTool, GetCurrentConfigTool
```

### Pattern 3: Asynchronous Task Execution

```python
from serena.agent import SerenaAgent

agent = SerenaAgent(project="/path/to/project")

# Issue task for asynchronous execution (returns Future)
future = agent.issue_task(
    lambda: some_long_running_operation(),
    name="MyCustomTask"
)

# Do other work...

# Wait for result
result = future.result(timeout=60)  # timeout in seconds
```

---

## Advanced Integration Patterns

### Pattern 4: Custom Script with Multiple Tools

```python
import json
from serena.agent import SerenaAgent
from serena.config.serena_config import SerenaConfig
from serena.tools import (
    FindSymbolTool,
    FindReferencingSymbolsTool,
    SearchForPatternTool
)

def analyze_symbol_usage(project_path: str, symbol_name: str):
    """Analyze how a symbol is used across the codebase."""
    
    # Initialize agent
    agent = SerenaAgent(
        project=project_path,
        serena_config=SerenaConfig(
            gui_log_window_enabled=False,
            web_dashboard=False,
            log_level=20  # INFO level
        )
    )
    
    # Get tool instances
    find_symbol = agent.get_tool(FindSymbolTool)
    find_refs = agent.get_tool(FindReferencingSymbolsTool)
    search = agent.get_tool(SearchForPatternTool)
    
    # Find the symbol definition
    print(f"Finding symbol: {symbol_name}")
    symbol_result = agent.execute_task(
        lambda: find_symbol.apply(
            name_path=symbol_name,
            depth=1,
            include_body=True
        )
    )
    symbols = json.loads(symbol_result)
    
    if not symbols:
        print(f"Symbol '{symbol_name}' not found!")
        return
    
    # Analyze each matching symbol
    for symbol in symbols:
        file_path = symbol.get('relative_path')
        print(f"\nSymbol found in: {file_path}")
        
        # Find references to this symbol
        refs_result = agent.execute_task(
            lambda: find_refs.apply(
                name_path=symbol_name,
                relative_path=file_path
            )
        )
        references = json.loads(refs_result)
        
        print(f"Found {len(references)} references")
        
        # Show reference details
        for ref in references[:5]:  # First 5 refs
            print(f"  - {ref['relative_path']}: line {ref['body_location']['start_line']}")

if __name__ == "__main__":
    analyze_symbol_usage("/path/to/project", "MyClass")
```

### Pattern 5: Batch Processing Multiple Files

```python
from serena.agent import SerenaAgent
from serena.tools import GetSymbolsOverviewTool
import json

def generate_symbol_index(project_path: str, output_file: str):
    """Generate an index of all symbols in the project."""
    
    agent = SerenaAgent(project=project_path)
    overview_tool = agent.get_tool(GetSymbolsOverviewTool)
    
    # Get all source files
    project = agent.get_active_project()
    source_files = project.gather_source_files()
    
    # Build index
    index = {}
    for file_path in source_files:
        try:
            result = agent.execute_task(
                lambda fp=file_path: overview_tool.apply(fp)
            )
            symbols = json.loads(result)
            index[file_path] = symbols
            print(f"Indexed: {file_path} ({len(symbols)} symbols)")
        except Exception as e:
            print(f"Error indexing {file_path}: {e}")
    
    # Save index
    with open(output_file, 'w') as f:
        json.dump(index, f, indent=2)
    
    print(f"\nIndex saved to: {output_file}")
```

---

## Tool Execution

### Direct Tool Execution (apply_ex)

```python
from serena.agent import SerenaAgent
from serena.tools import ReadFileTool

agent = SerenaAgent(project="/path/to/project")
read_tool = agent.get_tool(ReadFileTool)

# Direct execution with error handling
result = read_tool.apply_ex(
    relative_path="src/main.py",
    start_line=0,
    end_line=50,
    log_call=True,          # Log the tool call
    catch_exceptions=True   # Handle exceptions gracefully
)

print(result)
```

### Task Executor Pattern (Recommended)

The agent's task executor ensures **linear, thread-safe execution**:

```python
agent = SerenaAgent(project="/path/to/project")

# All tasks execute sequentially, even if issued simultaneously
future1 = agent.issue_task(lambda: tool1.apply(arg1))
future2 = agent.issue_task(lambda: tool2.apply(arg2))
future3 = agent.issue_task(lambda: tool3.apply(arg3))

# Tasks execute in order: tool1 -> tool2 -> tool3

result1 = future1.result()  # Blocks until tool1 completes
result2 = future2.result()  # Blocks until tool2 completes
result3 = future3.result()  # Blocks until tool3 completes
```

### Getting Tools by Name

```python
# By class
from serena.tools import FindSymbolTool
tool = agent.get_tool(FindSymbolTool)

# By name (string)
tool = agent.get_tool_by_name("find_symbol")
```

### Listing Active Tools

```python
# Get active tool names
active_tools = agent.get_active_tool_names()
print(f"Active tools: {', '.join(active_tools)}")

# Check if a specific tool is active
is_active = agent.tool_is_active("find_symbol")
is_active = agent.tool_is_active(FindSymbolTool)  # Also works
```

---

## Project Management

### Loading and Activating Projects

```python
from serena.agent import SerenaAgent

# Method 1: Activate on initialization
agent = SerenaAgent(project="/path/to/project")

# Method 2: Activate after creation
agent = SerenaAgent(project=None)
project = agent.activate_project_from_path_or_name("/path/to/project")

# Method 3: Switch between projects
agent.activate_project_from_path_or_name("project_name")
agent.activate_project_from_path_or_name("/different/path")
```

### Working with Projects

```python
# Get active project
project = agent.get_active_project()

if project:
    print(f"Project: {project.project_name}")
    print(f"Root: {project.project_root}")
    print(f"Language: {project.project_config.language}")
    
    # Read project files
    content = project.read_file("src/main.py")
    
    # Check if path is ignored
    is_ignored = project.is_ignored_path("node_modules/package.json")
    
    # Get source files
    source_files = project.gather_source_files()
```

### Project Configuration

Projects are configured via `project.yml`:

```yaml
language: python  # Language for LSP
name: my-project
encoding: utf-8

# Tool inclusions/exclusions
include_tools:
  - find_symbol
  - read_file
exclude_tools:
  - execute_shell_command

# Read-only mode
read_only: false

# Initial prompt
initial_prompt: |
  This project implements a REST API using FastAPI.
  The main entry point is in src/main.py.
```

---

## Configuration

### SerenaConfig

The global configuration class:

```python
from serena.config.serena_config import SerenaConfig

config = SerenaConfig(
    # Dashboard settings
    web_dashboard=True,
    web_dashboard_open_on_launch=False,
    gui_log_window_enabled=False,
    
    # Logging
    log_level=20,  # INFO
    trace_lsp_communication=False,
    
    # Tool execution
    tool_timeout=300,  # seconds
    default_max_tool_answer_chars=50000,
    
    # Performance
    record_tool_usage_stats=True,
    
    # Mode
    jetbrains=False  # Use LSP instead of JetBrains plugin
)

agent = SerenaAgent(project="/path", serena_config=config)
```

### Contexts

Contexts define the operational environment:

```python
from serena.config.context_mode import SerenaAgentContext

# Load built-in context
context = SerenaAgentContext.load("agent")

# Load custom context
context = SerenaAgentContext.load("/path/to/custom_context.yml")

# Use context
agent = SerenaAgent(
    project="/path",
    context=context
)
```

Available built-in contexts:
- `agent`: General-purpose coding agent
- `ide_assistant`: IDE integration mode

### Modes

Modes modify available tools and prompts:

```python
from serena.config.context_mode import SerenaAgentMode

# Load default modes
modes = SerenaAgentMode.load_default_modes()

# Load specific modes
modes = [
    SerenaAgentMode.load("editing"),
    SerenaAgentMode.load("interactive")
]

# Use modes
agent = SerenaAgent(project="/path", modes=modes)

# Switch modes dynamically
agent.set_modes([
    SerenaAgentMode.load("planning"),
    SerenaAgentMode.load("one-shot")
])
```

Available built-in modes:
- `editing`: Enables editing tools
- `interactive`: Interactive workflow tools
- `planning`: Planning and analysis focus
- `one-shot`: Single-task execution

---

## Agent Framework Integration (Agno Example)

Serena can be integrated into existing agent frameworks. Here's how it's done with **Agno**:

```python
from agno.agent import Agent
from agno.models.anthropic.claude import Claude
from agno.tools.function import Function
from agno.tools.toolkit import Toolkit

from serena.agent import SerenaAgent, Tool
from serena.config.context_mode import SerenaAgentContext

class SerenaAgnoToolkit(Toolkit):
    """Wraps Serena tools as Agno functions."""
    
    def __init__(self, serena_agent: SerenaAgent):
        super().__init__("Serena")
        
        # Convert each Serena tool to an Agno function
        for tool in serena_agent.get_exposed_tool_instances():
            self.functions[tool.get_name_from_cls()] = self._create_agno_function(tool)
    
    @staticmethod
    def _create_agno_function(tool: Tool) -> Function:
        """Convert a Serena tool to an Agno function."""
        
        def entrypoint(**kwargs):
            # Execute the tool via Serena's API
            return tool.apply_ex(log_call=True, catch_exceptions=True, **kwargs)
        
        # Create Agno function from the tool's apply method
        function = Function.from_callable(tool.get_apply_fn())
        function.name = tool.get_name_from_cls()
        function.entrypoint = entrypoint
        function.skip_entrypoint_processing = True
        
        return function

# Create Serena agent
serena_agent = SerenaAgent(
    project="/path/to/project",
    context=SerenaAgentContext.load("agent")
)

# Wrap tools in Agno toolkit
serena_toolkit = SerenaAgnoToolkit(serena_agent)

# Create Agno agent with Serena tools
agno_agent = Agent(
    name="Serena",
    model=Claude(id="claude-3-7-sonnet-20250219"),
    tools=[serena_toolkit],
    system_message=serena_agent.create_system_prompt(),
    markdown=True
)

# Use the agent
response = agno_agent.run("Analyze the main function in src/app.py")
print(response.content)
```

### Integration Patterns for Other Frameworks

#### LangChain Integration Concept

```python
from langchain.tools import Tool as LangChainTool
from serena.agent import SerenaAgent

def create_langchain_tool(serena_tool):
    """Convert Serena tool to LangChain tool."""
    return LangChainTool(
        name=serena_tool.get_name_from_cls(),
        description=serena_tool.get_apply_docstring(),
        func=lambda **kwargs: serena_tool.apply_ex(**kwargs)
    )

serena_agent = SerenaAgent(project="/path")
langchain_tools = [
    create_langchain_tool(tool)
    for tool in serena_agent.get_exposed_tool_instances()
]
```

#### AutoGPT Integration Concept

```python
from autogpt.agents import Agent
from serena.agent import SerenaAgent

class SerenaAutoGPTPlugin:
    def __init__(self, project_path: str):
        self.serena = SerenaAgent(project=project_path)
    
    def get_commands(self):
        """Return commands for AutoGPT."""
        commands = {}
        for tool in self.serena.get_exposed_tool_instances():
            commands[tool.get_name_from_cls()] = (
                tool.get_apply_docstring(),
                lambda **kw: tool.apply_ex(**kw)
            )
        return commands
```

---

## API Reference

### SerenaAgent Core Methods

#### Initialization

```python
SerenaAgent(
    project: str | None = None,
    project_activation_callback: Callable[[], None] | None = None,
    serena_config: SerenaConfig | None = None,
    context: SerenaAgentContext | None = None,
    modes: list[SerenaAgentMode] | None = None,
    memory_log_handler: MemoryLogHandler | None = None
)
```

#### Tool Access

```python
agent.get_tool(tool_class: type[Tool]) -> Tool
agent.get_tool_by_name(tool_name: str) -> Tool
agent.get_exposed_tool_instances() -> list[Tool]
agent.get_active_tool_names() -> list[str]
agent.tool_is_active(tool_class: type[Tool] | str) -> bool
```

#### Task Execution

```python
agent.issue_task(task: Callable[[], Any], name: str | None = None) -> Future
agent.execute_task(task: Callable[[], T]) -> T
```

#### Project Management

```python
agent.get_active_project() -> Project | None
agent.get_active_project_or_raise() -> Project
agent.activate_project_from_path_or_name(project: str) -> Project
agent.load_project_from_path_or_name(project: str, autogenerate: bool) -> Project | None
agent.get_project_root() -> str
```

#### Configuration

```python
agent.get_context() -> SerenaAgentContext
agent.get_active_modes() -> list[SerenaAgentMode]
agent.set_modes(modes: list[SerenaAgentMode]) -> None
agent.create_system_prompt() -> str
agent.get_current_config_overview() -> str
```

#### Language Server

```python
agent.is_using_language_server() -> bool
agent.is_language_server_running() -> bool
agent.reset_language_server() -> None
```

### Tool Base Class

All tools inherit from `Tool` and implement:

```python
class MyCustomTool(Tool):
    """Tool description goes here."""
    
    def apply(self, param1: str, param2: int = 0) -> str:
        """
        Detailed description of what the tool does.
        
        :param param1: Description of param1
        :param param2: Description of param2
        :return: Description of return value
        """
        # Tool implementation
        return "result"
```

---

## Examples

### Example 1: Batch Symbol Analysis

```python
"""Analyze all classes in a project and report their complexity."""

from serena.agent import SerenaAgent
from serena.tools import FindSymbolTool
from serena.config.serena_config import SerenaConfig
import json

def analyze_classes(project_path: str):
    agent = SerenaAgent(
        project=project_path,
        serena_config=SerenaConfig(
            gui_log_window_enabled=False,
            web_dashboard=False
        )
    )
    
    find_tool = agent.get_tool(FindSymbolTool)
    
    # Find all classes
    result = agent.execute_task(
        lambda: find_tool.apply(
            name_path="",  # Empty = all symbols
            include_kinds=[5],  # 5 = Class
            depth=1,  # Include methods
            substring_matching=False
        )
    )
    
    classes = json.loads(result)
    
    # Analyze each class
    for cls in classes:
        name = cls['name_path']
        methods = len(cls.get('children', []))
        file = cls['relative_path']
        
        print(f"{name} ({file}): {methods} methods")
        
        if methods > 20:
            print(f"  ⚠️  High complexity!")

if __name__ == "__main__":
    analyze_classes("/path/to/project")
```

### Example 2: Custom Code Migration Tool

```python
"""Replace deprecated API usage across a codebase."""

from serena.agent import SerenaAgent
from serena.tools import SearchForPatternTool, ReplaceRegexTool
from serena.config.serena_config import SerenaConfig
import json

def migrate_api_usage(project_path: str, old_api: str, new_api: str):
    agent = SerenaAgent(
        project=project_path,
        serena_config=SerenaConfig(web_dashboard=False)
    )
    
    search_tool = agent.get_tool(SearchForPatternTool)
    replace_tool = agent.get_tool(ReplaceRegexTool)
    
    # Find all usages
    print(f"Searching for '{old_api}'...")
    result = agent.execute_task(
        lambda: search_tool.apply(
            substring_pattern=old_api,
            restrict_search_to_code_files=True
        )
    )
    
    matches = json.loads(result)
    total_files = sum(len(file_matches) for file_matches in matches.values())
    
    print(f"Found {total_files} files with '{old_api}'")
    
    # Replace in each file
    for file_path, file_matches in matches.items():
        if file_matches:
            print(f"Migrating {file_path}...")
            agent.execute_task(
                lambda: replace_tool.apply(
                    relative_path=file_path,
                    regex=old_api,
                    repl=new_api,
                    allow_multiple_occurrences=True
                )
            )
    
    print("Migration complete!")

if __name__ == "__main__":
    migrate_api_usage(
        "/path/to/project",
        old_api="old_function",
        new_api="new_function"
    )
```

### Example 3: Documentation Generator

```python
"""Generate documentation for all public functions."""

from serena.agent import SerenaAgent
from serena.tools import FindSymbolTool, CreateTextFileTool
from serena.config.serena_config import SerenaConfig
import json

def generate_api_docs(project_path: str, output_file: str):
    agent = SerenaAgent(project=project_path)
    
    find_tool = agent.get_tool(FindSymbolTool)
    create_tool = agent.get_tool(CreateTextFileTool)
    
    # Find all public functions
    result = agent.execute_task(
        lambda: find_tool.apply(
            name_path="",
            include_kinds=[12],  # 12 = Function
            include_body=True,
            depth=0
        )
    )
    
    functions = json.loads(result)
    
    # Build documentation
    docs = "# API Documentation\n\n"
    
    for func in functions:
        name = func['name_path']
        
        # Skip private functions
        if name.startswith('_'):
            continue
        
        file = func['relative_path']
        body = func.get('body', '')
        
        # Extract docstring from body
        docstring = ""
        if '"""' in body:
            start = body.find('"""') + 3
            end = body.find('"""', start)
            if end > start:
                docstring = body[start:end].strip()
        
        docs += f"## {name}\n\n"
        docs += f"**File**: `{file}`\n\n"
        if docstring:
            docs += f"{docstring}\n\n"
        docs += "---\n\n"
    
    # Save documentation
    agent.execute_task(
        lambda: create_tool.apply(
            relative_path=output_file,
            content=docs
        )
    )
    
    print(f"Documentation generated: {output_file}")

if __name__ == "__main__":
    generate_api_docs("/path/to/project", "docs/API.md")
```

---

## Next Steps

- **Custom Tool Development**: Learn how to create your own tools (see `CUSTOM_TOOLS.md`)
- **CLI Integration**: Build custom CLI commands (see `CLI_USAGE.md`)
- **Language Server Deep Dive**: Direct LSP access (see `LANGUAGE_SERVER_API.md`)
- **Dashboard Integration**: Embed the web dashboard (see `DASHBOARD_API.md`)

---

## Additional Resources

- **Demo Scripts**: See `scripts/demo_run_tools.py` for more examples
- **Agno Integration**: See `scripts/agno_agent.py` and `src/serena/agno.py`
- **Tool Implementations**: Browse `src/serena/tools/` for tool examples
- **Test Suite**: See `test/serena/` for comprehensive usage patterns

---

**Note**: This document covers using Serena as a Python library. For MCP server usage, see the main README.md.
