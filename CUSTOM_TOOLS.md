# Custom Tool Development Guide

This guide explains how to create custom tools that extend Serena's capabilities. Custom tools integrate seamlessly with Serena's agent system, MCP server, and agent framework integrations.

## Table of Contents

1. [Overview](#overview)
2. [Tool Architecture](#tool-architecture)
3. [Creating Your First Tool](#creating-your-first-tool)
4. [Tool Markers](#tool-markers)
5. [Accessing Agent Resources](#accessing-agent-resources)
6. [Advanced Tool Patterns](#advanced-tool-patterns)
7. [Tool Registration](#tool-registration)
8. [Best Practices](#best-practices)
9. [Complete Examples](#complete-examples)

---

## Overview

Custom tools in Serena are Python classes that:

- **Inherit from `Tool`** base class
- **Implement an `apply()` method** with typed parameters and docstring
- **Return a string result** (typically JSON for structured data)
- **Are automatically registered** when placed in the `serena.tools` module
- **Integrate with MCP** server, agent frameworks, and CLI

### Why Create Custom Tools?

- **Domain-specific operations**: Add operations specific to your codebase or workflow
- **Custom analysis**: Implement specialized code analysis or metrics
- **Integration**: Connect Serena to external systems or databases
- **Workflow automation**: Combine existing operations in new ways
- **Team-specific utilities**: Create tools tailored to your team's needs

---

## Tool Architecture

### Tool Base Class

All tools inherit from the `Tool` class, which itself inherits from `Component`:

```python
from serena.tools import Tool

class MyCustomTool(Tool):
    """
    Short description of what the tool does.
    This docstring becomes the tool's description shown to LLMs and users.
    """
    
    def apply(self, param1: str, param2: int = 0) -> str:
        """
        Detailed description of the tool's operation.
        
        This docstring is critical - it's what the LLM sees to understand
        how to use the tool. Be clear, concise, and precise.
        
        :param param1: Description of param1
        :param param2: Description of param2 with default value
        :return: Description of what the tool returns
        """
        # Implementation
        return "result"
```

### Tool Lifecycle

1. **Class Definition**: Tool class is defined in `src/serena/tools/`
2. **Automatic Registration**: `ToolRegistry` discovers all Tool subclasses at startup
3. **Instantiation**: `SerenaAgent` creates instances of all tools
4. **Activation**: Tools are activated based on context, modes, and project config
5. **Execution**: LLM or code calls `tool.apply(**params)` or `tool.apply_ex(**params)`

### Key Methods

| Method | Purpose | When to Override |
|--------|---------|------------------|
| `apply()` | **REQUIRED**: Core tool logic | Always |
| `get_name()` | Tool name (auto-generated from class name) | Rarely |
| `get_tool_description()` | Tool description (from class docstring) | Rarely |
| `apply_ex()` | Execution wrapper with error handling | Never |
| `is_active()` | Check if tool is currently active | Never |

---

## Creating Your First Tool

### Step 1: Basic Structure

Create a new file in `src/serena/tools/` (e.g., `my_custom_tools.py`):

```python
"""
My custom tools for Serena.
"""

from serena.tools import Tool
import json

class HelloWorldTool(Tool):
    """
    A simple example tool that says hello.
    """
    
    def apply(self, name: str) -> str:
        """
        Say hello to someone.
        
        :param name: The name to greet
        :return: A greeting message
        """
        return json.dumps({"message": f"Hello, {name}!"})
```

**That's it!** The tool is now:
- ✅ Automatically registered
- ✅ Available in the agent
- ✅ Exposed via MCP server
- ✅ Usable in agent frameworks

### Step 2: Using Your Tool

```python
from serena.agent import SerenaAgent
from my_custom_tools import HelloWorldTool

agent = SerenaAgent(project="/path/to/project")
tool = agent.get_tool(HelloWorldTool)
result = agent.execute_task(lambda: tool.apply(name="World"))
print(result)  # {"message": "Hello, World!"}
```

### Step 3: Tool Naming Convention

Tool names are automatically derived from class names:

| Class Name | Tool Name |
|------------|-----------|
| `HelloWorldTool` | `hello_world` |
| `FindSymbolTool` | `find_symbol` |
| `MyCustomAnalysisTool` | `my_custom_analysis` |

**Rule**: Class name → remove "Tool" suffix → convert CamelCase to snake_case

---

## Tool Markers

Tool markers are mixin classes that modify tool behavior and control when tools are available.

### Available Markers

#### 1. `ToolMarkerCanEdit`

**Purpose**: Marks tools that modify code or files.

**Effect**: 
- Tool is disabled when project is in read-only mode
- Tool can be filtered out by `ToolSet.without_editing_tools()`

```python
from serena.tools import Tool, ToolMarkerCanEdit

class CreateFileTool(Tool, ToolMarkerCanEdit):
    """Creates or overwrites a file."""
    
    def apply(self, path: str, content: str) -> str:
        # ... implementation
        pass
```

#### 2. `ToolMarkerSymbolicRead`

**Purpose**: Marks tools that perform symbolic/semantic read operations via language server.

**Effect**: Categorizes the tool for filtering and organization.

```python
from serena.tools import Tool, ToolMarkerSymbolicRead

class FindSymbolTool(Tool, ToolMarkerSymbolicRead):
    """Finds symbols in the codebase."""
    
    def apply(self, name: str) -> str:
        # Use language server for semantic search
        symbol_retriever = self.create_language_server_symbol_retriever()
        # ... implementation
        pass
```

#### 3. `ToolMarkerSymbolicEdit`

**Purpose**: Marks tools that perform symbolic/semantic edit operations.

**Effect**: Inherits from `ToolMarkerCanEdit`, so it's also considered an editing tool.

```python
from serena.tools import Tool, ToolMarkerSymbolicEdit

class ReplaceSymbolBodyTool(Tool, ToolMarkerSymbolicEdit):
    """Replaces the body of a symbol."""
    
    def apply(self, name_path: str, body: str) -> str:
        code_editor = self.create_code_editor()
        # ... implementation
        pass
```

#### 4. `ToolMarkerOptional`

**Purpose**: Marks tools as disabled by default.

**Effect**: Tool must be explicitly enabled in configuration to be active.

```python
from serena.tools import Tool, ToolMarkerOptional

class AdvancedAnalysisTool(Tool, ToolMarkerOptional):
    """
    Advanced analysis that might be expensive or rarely needed.
    """
    
    def apply(self, target: str) -> str:
        # ... implementation
        pass
```

#### 5. `ToolMarkerDoesNotRequireActiveProject`

**Purpose**: Tool can run without an active project.

**Effect**: Tool works even when no project is loaded.

```python
from serena.tools import Tool, ToolMarkerDoesNotRequireActiveProject

class GetConfigTool(Tool, ToolMarkerDoesNotRequireActiveProject):
    """Returns current configuration."""
    
    def apply(self) -> str:
        return self.agent.get_current_config_overview()
```

### Combining Markers

You can use multiple markers:

```python
from serena.tools import (
    Tool, 
    ToolMarkerSymbolicEdit, 
    ToolMarkerOptional
)

class ExperimentalEditTool(Tool, ToolMarkerSymbolicEdit, ToolMarkerOptional):
    """
    Experimental editing tool - disabled by default and can edit code.
    """
    
    def apply(self, params: str) -> str:
        # ... implementation
        pass
```

---

## Accessing Agent Resources

Tools inherit from `Component`, which provides access to agent resources through properties.

### 1. Project Access

```python
class MyProjectTool(Tool):
    def apply(self) -> str:
        # Get active project
        project = self.project  # Returns Project instance
        
        # Get project root path
        root = self.get_project_root()
        
        # Read a file
        content = project.read_file("src/main.py")
        
        # Check if path is ignored
        is_ignored = project.is_ignored_path("node_modules/pkg.json")
        
        # Get source files
        files = project.gather_source_files()
        
        return f"Project: {project.project_name} at {root}"
```

### 2. Language Server Access

```python
class MySymbolTool(Tool, ToolMarkerSymbolicRead):
    def apply(self, relative_path: str) -> str:
        # Create symbol retriever (uses language server)
        symbol_retriever = self.create_language_server_symbol_retriever()
        
        # Get symbol overview
        overview = symbol_retriever.get_symbol_overview(relative_path)
        
        # Find symbols by name
        symbols = symbol_retriever.find_by_name(
            "MyClass",
            within_relative_path=relative_path
        )
        
        return json.dumps([s.to_dict() for s in symbols])
```

### 3. Code Editor Access

```python
class MyEditTool(Tool, ToolMarkerSymbolicEdit):
    def apply(self, path: str, name: str, new_body: str) -> str:
        # Create code editor (handles symbol-level edits)
        code_editor = self.create_code_editor()
        
        # Replace symbol body
        code_editor.replace_body(
            name_path=name,
            relative_file_path=path,
            body=new_body
        )
        
        return "OK"
```

### 4. Memory Manager Access

```python
class MyMemoryTool(Tool):
    def apply(self, name: str) -> str:
        # Access memory manager
        manager = self.memories_manager
        
        # Save memory
        manager.save_memory(name, "content")
        
        # Load memory
        content = manager.load_memory(name)
        
        # List memories
        memories = manager.list_memories()
        
        return json.dumps(memories)
```

### 5. Agent Access

```python
class MyAgentTool(Tool):
    def apply(self) -> str:
        # Access the agent directly
        agent = self.agent
        
        # Get active tool names
        tools = agent.get_active_tool_names()
        
        # Get another tool instance
        other_tool = agent.get_tool(SomeOtherTool)
        
        # Check if project is loaded
        project = agent.get_active_project()
        if project:
            return f"Project: {project.project_name}"
        
        return "No active project"
```

### 6. Prompt Factory Access

```python
class MyPromptTool(Tool):
    def apply(self) -> str:
        # Access prompt factory
        factory = self.prompt_factory
        
        # Create custom prompts
        prompt = factory.create_onboarding_prompt(system="Windows")
        
        return prompt
```

### 7. Lines Read Tracker

```python
class MyReadTool(Tool):
    def apply(self, path: str, start: int, end: int) -> str:
        # Track which lines were read (for validation in edit tools)
        self.lines_read.add_lines_read(path, (start, end))
        
        # Check if lines were read
        were_read = self.lines_read.were_lines_read(path, (start, end))
        
        return f"Lines {start}-{end} read: {were_read}"
```

### 8. Result Length Limiting

```python
class MyDataTool(Tool):
    def apply(self, query: str, max_answer_chars: int = -1) -> str:
        # Fetch large result
        result = fetch_large_data(query)
        
        # Automatically limit based on config or parameter
        return self._limit_length(result, max_answer_chars)
```

---

## Advanced Tool Patterns

### Pattern 1: File Editing with Context Manager

Use `EditedFileContext` for safe file editing:

```python
from serena.tools import Tool, ToolMarkerCanEdit, EditedFileContext
import re

class MyReplaceTool(Tool, ToolMarkerCanEdit):
    """Replaces text in a file safely."""
    
    def apply(self, path: str, old: str, new: str) -> str:
        self.project.validate_relative_path(path)
        
        with EditedFileContext(path, self.agent) as context:
            # Get original content
            original = context.get_original_content()
            
            # Perform replacement
            updated = original.replace(old, new)
            
            # Set updated content (written on successful exit)
            context.set_updated_content(updated)
        
        return "OK"
```

### Pattern 2: Multi-Step Operations

```python
class MyAnalysisTool(Tool):
    """Performs multi-step analysis."""
    
    def apply(self, target: str) -> str:
        results = []
        
        # Step 1: Find symbols
        symbol_retriever = self.create_language_server_symbol_retriever()
        symbols = symbol_retriever.find_by_name(target)
        
        # Step 2: Analyze each symbol
        for symbol in symbols:
            # Get symbol body
            file_path = symbol.location.relative_path
            content = self.project.read_file(file_path)
            
            # Analyze
            analysis = self._analyze_symbol(symbol, content)
            results.append(analysis)
        
        return json.dumps(results)
    
    def _analyze_symbol(self, symbol, content):
        # Helper method
        return {
            "name": symbol.name,
            "lines": symbol.body_location.end_line - symbol.body_location.start_line
        }
```

### Pattern 3: Tool Composition

```python
class MyCompositeTool(Tool):
    """Combines multiple existing tools."""
    
    def apply(self, target: str) -> str:
        # Get other tools
        find_tool = self.agent.get_tool(FindSymbolTool)
        read_tool = self.agent.get_tool(ReadFileTool)
        
        # Use tools together
        symbols_json = find_tool.apply(name_path=target)
        symbols = json.loads(symbols_json)
        
        results = []
        for symbol in symbols:
            file_content = read_tool.apply(symbol['relative_path'])
            results.append({
                "symbol": symbol['name_path'],
                "file": symbol['relative_path'],
                "content_length": len(file_content)
            })
        
        return json.dumps(results)
```

### Pattern 4: Configurable Tools

```python
class MyConfigurableTool(Tool):
    """Tool with configurable behavior."""
    
    def apply(
        self, 
        target: str,
        mode: str = "fast",
        max_results: int = 100
    ) -> str:
        """
        Analyze target with configurable options.
        
        :param target: What to analyze
        :param mode: Analysis mode ("fast", "thorough", "deep")
        :param max_results: Maximum results to return
        :return: Analysis results
        """
        if mode == "fast":
            results = self._fast_analysis(target)
        elif mode == "thorough":
            results = self._thorough_analysis(target)
        elif mode == "deep":
            results = self._deep_analysis(target)
        else:
            return f"Error: Unknown mode '{mode}'"
        
        # Limit results
        results = results[:max_results]
        
        return json.dumps(results)
```

### Pattern 5: Error Handling

```python
class MyRobustTool(Tool):
    """Tool with comprehensive error handling."""
    
    def apply(self, path: str) -> str:
        try:
            # Validate inputs
            if not path:
                return "Error: Path cannot be empty"
            
            self.project.validate_relative_path(path)
            
            # Perform operation
            result = self._process_file(path)
            
            # Validate output
            if not result:
                return "Error: No results found"
            
            return json.dumps(result)
            
        except FileNotFoundError as e:
            return f"Error: File not found: {path}"
        except ValueError as e:
            return f"Error: Invalid input: {e}"
        except Exception as e:
            # Log unexpected errors
            return f"Error: Unexpected error: {e}"
```

### Pattern 6: Streaming/Progressive Results

```python
class MyProgressiveTool(Tool):
    """Tool that processes files progressively."""
    
    def apply(self, pattern: str) -> str:
        files = self.project.gather_source_files()
        results = []
        processed = 0
        
        for file in files:
            try:
                result = self._process_file(file, pattern)
                if result:
                    results.append(result)
                processed += 1
                
                # Early exit if we have enough results
                if len(results) >= 10:
                    break
                    
            except Exception as e:
                # Continue on errors
                continue
        
        return json.dumps({
            "results": results,
            "processed": processed,
            "total": len(files)
        })
```

---

## Tool Registration

### Automatic Registration

Tools are automatically registered if:

1. **Module location**: File is in `src/serena/tools/`
2. **Inherits from Tool**: Class inherits from `Tool`
3. **Has unique name**: Tool name (derived from class name) is unique

### Registration Process

```python
# In src/serena/tools/my_tools.py

from serena.tools import Tool

class MyTool(Tool):
    """My custom tool."""
    
    def apply(self) -> str:
        return "Hello!"

# Automatically registered at Serena startup
# Tool name: "my_tool"
```

### Checking Registration

```python
from serena.tools import ToolRegistry

registry = ToolRegistry()

# Check if tool is registered
is_valid = registry.is_valid_tool_name("my_tool")

# Get all tool names
all_tools = registry.get_tool_names()

# Get tool class by name
tool_class = registry.get_tool_class_by_name("my_tool")
```

### Tool Discovery

The `ToolRegistry` uses reflection to discover all Tool subclasses:

```python
@singleton
class ToolRegistry:
    def __init__(self) -> None:
        self._tool_dict: dict[str, RegisteredTool] = {}
        for cls in iter_subclasses(Tool):
            # Only register tools in serena.tools module
            if not cls.__module__.startswith("serena.tools"):
                continue
            
            is_optional = issubclass(cls, ToolMarkerOptional)
            name = cls.get_name_from_cls()
            
            if name in self._tool_dict:
                raise ValueError(f"Duplicate tool name: {name}")
            
            self._tool_dict[name] = RegisteredTool(
                tool_class=cls,
                is_optional=is_optional,
                tool_name=name
            )
```

---

## Best Practices

### 1. Docstring Guidelines

**Class Docstring**: Brief, one-line description
```python
class MyTool(Tool):
    """Performs X operation on Y."""
```

**Apply Method Docstring**: Detailed, with parameter descriptions
```python
def apply(self, target: str, depth: int = 1) -> str:
    """
    Analyze the given target at the specified depth.
    
    This tool examines the target using semantic analysis
    and returns structured results. Use depth=1 for quick
    overview, depth=2+ for detailed analysis.
    
    :param target: The name or path to analyze
    :param depth: Analysis depth (1-5), default is 1
    :return: JSON object with analysis results
    """
```

### 2. Type Annotations

Always use type hints:
```python
def apply(
    self, 
    path: str,                    # Required parameter
    recursive: bool = False,       # Optional with default
    max_items: int | None = None  # Optional, can be None
) -> str:                          # Return type
```

### 3. Return Values

Always return strings (preferably JSON):
```python
# Good: Structured data as JSON
return json.dumps({"status": "success", "count": 42})

# Good: Simple success
return "OK"

# Good: Error message
return "Error: File not found"

# Bad: Non-string return
return {"data": 123}  # ❌
```

### 4. Error Messages

Provide clear, actionable error messages:
```python
# Bad
return "Error"

# Good
return "Error: File 'main.py' not found. Check if the path is correct relative to project root."

# Good
return "Error: Symbol 'MyClass' not found. Did you mean 'MyBaseClass'?"
```

### 5. Performance

Consider performance for tools that process many files:
```python
class MyScalableTool(Tool):
    def apply(self, pattern: str, max_results: int = 100) -> str:
        results = []
        
        for file in self.project.gather_source_files():
            # Early exit when we have enough
            if len(results) >= max_results:
                break
            
            # Process file
            result = self._process(file, pattern)
            if result:
                results.append(result)
        
        return json.dumps(results)
```

### 6. Project Validation

Always validate relative paths:
```python
def apply(self, path: str) -> str:
    # Validates path is within project and not ignored
    self.project.validate_relative_path(path)
    
    # Now safe to use
    content = self.project.read_file(path)
```

### 7. Language Server Checks

Check if language server is available:
```python
class MyLSTool(Tool, ToolMarkerSymbolicRead):
    def apply(self, target: str) -> str:
        if not self.agent.is_using_language_server():
            return "Error: Language server not available"
        
        symbol_retriever = self.create_language_server_symbol_retriever()
        # ... use symbol retriever
```

---

## Complete Examples

### Example 1: File Statistics Tool

```python
"""
File statistics tools.
"""

from serena.tools import Tool
import json
import os

class FileStatsTool(Tool):
    """
    Provides statistics about files in the project.
    """
    
    def apply(self, relative_path: str = ".", extension: str | None = None) -> str:
        """
        Calculate statistics for files in the given directory.
        
        :param relative_path: Directory to analyze (default: project root)
        :param extension: Filter by extension (e.g., ".py"), None for all files
        :return: JSON with file statistics
        """
        self.project.validate_relative_path(relative_path)
        
        files = self.project.gather_source_files()
        
        # Filter by extension if provided
        if extension:
            files = [f for f in files if f.endswith(extension)]
        
        # Calculate statistics
        stats = {
            "total_files": len(files),
            "total_lines": 0,
            "total_size": 0,
            "by_extension": {}
        }
        
        for file_path in files:
            try:
                full_path = os.path.join(self.project.project_root, file_path)
                
                # Get file size
                size = os.path.getsize(full_path)
                stats["total_size"] += size
                
                # Count lines
                content = self.project.read_file(file_path)
                lines = len(content.splitlines())
                stats["total_lines"] += lines
                
                # Track by extension
                ext = os.path.splitext(file_path)[1] or "no_extension"
                if ext not in stats["by_extension"]:
                    stats["by_extension"][ext] = {"count": 0, "lines": 0}
                
                stats["by_extension"][ext]["count"] += 1
                stats["by_extension"][ext]["lines"] += lines
                
            except Exception as e:
                # Skip files that can't be read
                continue
        
        return json.dumps(stats, indent=2)
```

### Example 2: Complexity Analysis Tool

```python
"""
Code complexity analysis tools.
"""

from serena.tools import Tool, ToolMarkerSymbolicRead
import json

class ComplexityAnalysisTool(Tool, ToolMarkerSymbolicRead):
    """
    Analyzes code complexity of classes and functions.
    """
    
    def apply(
        self, 
        relative_path: str | None = None,
        threshold: int = 20
    ) -> str:
        """
        Find classes or functions with high complexity.
        
        :param relative_path: File or directory to analyze (None for entire project)
        :param threshold: Complexity threshold (default: 20 methods)
        :return: JSON with high-complexity symbols
        """
        symbol_retriever = self.create_language_server_symbol_retriever()
        
        # Find all classes
        classes = symbol_retriever.find_by_name(
            "",  # Empty = all symbols
            include_kinds=[5],  # 5 = Class
            within_relative_path=relative_path or ""
        )
        
        high_complexity = []
        
        for cls in classes:
            # Get class with children
            cls_dict = cls.to_dict(depth=1)
            method_count = len(cls_dict.get("children", []))
            
            if method_count >= threshold:
                high_complexity.append({
                    "name": cls.name_path,
                    "file": cls.location.relative_path,
                    "line": cls.location.line,
                    "method_count": method_count,
                    "complexity": "high" if method_count > 30 else "medium"
                })
        
        # Sort by complexity
        high_complexity.sort(key=lambda x: x["method_count"], reverse=True)
        
        result = {
            "threshold": threshold,
            "found": len(high_complexity),
            "symbols": high_complexity[:20]  # Top 20
        }
        
        return json.dumps(result, indent=2)
```

### Example 3: Dependency Mapper Tool

```python
"""
Dependency analysis tools.
"""

from serena.tools import Tool, ToolMarkerSymbolicRead
from collections import defaultdict
import json
import re

class DependencyMapperTool(Tool, ToolMarkerSymbolicRead):
    """
    Maps dependencies between files based on imports.
    """
    
    def apply(self, target_file: str | None = None) -> str:
        """
        Create a dependency map showing which files import others.
        
        :param target_file: Specific file to analyze, None for entire project
        :return: JSON with dependency graph
        """
        files = [target_file] if target_file else self.project.gather_source_files()
        
        # Filter to Python files for this example
        files = [f for f in files if f.endswith('.py')]
        
        dependencies = defaultdict(list)
        
        for file_path in files:
            try:
                content = self.project.read_file(file_path)
                imports = self._extract_imports(content)
                
                # Try to resolve imports to files in project
                for imp in imports:
                    resolved = self._resolve_import(imp)
                    if resolved and resolved in files:
                        dependencies[file_path].append(resolved)
                        
            except Exception:
                continue
        
        # Calculate metrics
        result = {
            "files_analyzed": len(files),
            "dependencies": dict(dependencies),
            "metrics": {
                "most_imported": self._most_imported(dependencies),
                "most_dependencies": self._most_dependencies(dependencies)
            }
        }
        
        return json.dumps(result, indent=2)
    
    def _extract_imports(self, content: str) -> list[str]:
        """Extract import statements from Python code."""
        imports = []
        for line in content.splitlines():
            # Match: import X or from X import Y
            if match := re.match(r'^(?:from\s+([\w.]+)|import\s+([\w.]+))', line.strip()):
                module = match.group(1) or match.group(2)
                imports.append(module)
        return imports
    
    def _resolve_import(self, module: str) -> str | None:
        """Try to resolve module to project file."""
        # Simple resolution: module.name -> module/name.py
        path = module.replace('.', '/') + '.py'
        if path in self.project.gather_source_files():
            return path
        return None
    
    def _most_imported(self, deps: dict) -> list[dict]:
        """Find most imported files."""
        import_counts = defaultdict(int)
        for imports in deps.values():
            for imp in imports:
                import_counts[imp] += 1
        
        sorted_imports = sorted(import_counts.items(), key=lambda x: x[1], reverse=True)
        return [{"file": f, "count": c} for f, c in sorted_imports[:5]]
    
    def _most_dependencies(self, deps: dict) -> list[dict]:
        """Find files with most dependencies."""
        sorted_deps = sorted(deps.items(), key=lambda x: len(x[1]), reverse=True)
        return [{"file": f, "count": len(imports)} for f, imports in sorted_deps[:5]]
```

### Example 4: Test Coverage Tool

```python
"""
Test coverage analysis tools.
"""

from serena.tools import Tool, ToolMarkerSymbolicRead
import json
import os

class TestCoverageTool(Tool, ToolMarkerSymbolicRead):
    """
    Analyzes test coverage by matching test files with source files.
    """
    
    def apply(
        self, 
        source_dir: str = "src",
        test_dir: str = "test"
    ) -> str:
        """
        Check which source files have corresponding test files.
        
        :param source_dir: Directory containing source code
        :param test_dir: Directory containing tests
        :return: JSON with coverage analysis
        """
        # Get all source files
        all_files = self.project.gather_source_files()
        source_files = [f for f in all_files if f.startswith(source_dir)]
        test_files = [f for f in all_files if f.startswith(test_dir)]
        
        covered = []
        uncovered = []
        
        for source_file in source_files:
            # Look for corresponding test file
            base_name = os.path.basename(source_file)
            test_name = f"test_{base_name}"
            
            has_test = any(test_name in tf for tf in test_files)
            
            if has_test:
                covered.append(source_file)
            else:
                uncovered.append(source_file)
        
        coverage_percent = (len(covered) / len(source_files) * 100) if source_files else 0
        
        result = {
            "summary": {
                "total_source_files": len(source_files),
                "covered": len(covered),
                "uncovered": len(uncovered),
                "coverage_percent": round(coverage_percent, 2)
            },
            "uncovered_files": uncovered[:10],  # Top 10 uncovered
            "recommendations": self._get_recommendations(uncovered)
        }
        
        return json.dumps(result, indent=2)
    
    def _get_recommendations(self, uncovered: list[str]) -> list[str]:
        """Generate recommendations for improving coverage."""
        if not uncovered:
            return ["All source files have corresponding tests!"]
        
        return [
            f"Create tests for {len(uncovered)} uncovered files",
            f"Priority: {', '.join(uncovered[:3])}",
            "Consider using pytest or unittest for test implementation"
        ]
```

---

## Next Steps

- **Deploy Your Tools**: Add them to `src/serena/tools/` and restart Serena
- **Test Thoroughly**: Use `scripts/demo_run_tools.py` as a template
- **Document Well**: Clear docstrings make tools more useful to LLMs
- **Share**: Consider contributing useful tools back to the Serena project

---

## Additional Resources

- **Tool Base Class**: See `src/serena/tools/tools_base.py`
- **Existing Tools**: Browse `src/serena/tools/` for inspiration
- **Component Class**: See `src/serena/tools/tools_base.py` for available properties
- **Python Library API**: See `PYTHON_LIBRARY_API.md` for agent integration

---

**Happy Tool Building!** 🛠️
