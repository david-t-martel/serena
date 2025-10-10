#!/usr/bin/env python3
"""
Analyze Serena using its own semantic tools to gather comprehensive insights.
This script demonstrates Serena's capabilities beyond MCP by using it as a Python library.
"""

import json
import logging
from pathlib import Path

from serena.agent import SerenaAgent
from serena.config.serena_config import SerenaConfig
from serena.tools import (
    FindSymbolTool,
    FindReferencingSymbolsTool,
    GetSymbolsOverviewTool,
    SearchForPatternTool,
    ListDirTool
)

# Suppress verbose logging
logging.basicConfig(level=logging.WARNING)

def analyze_serena():
    """Use Serena's tools to analyze the serena-source project."""
    
    print("=" * 80)
    print("SERENA SELF-ANALYSIS: Using Serena Tools to Examine Serena")
    print("=" * 80)
    print()
    
    # Initialize agent without GUI/dashboard
    agent = SerenaAgent(
        project="T:/projects/serena-source",
        serena_config=SerenaConfig(
            gui_log_window_enabled=False,
            web_dashboard=False,
            log_level=logging.WARNING
        )
    )
    
    # Get tool instances
    find_symbol = agent.get_tool(FindSymbolTool)
    find_refs = agent.get_tool(FindReferencingSymbolsTool)
    get_overview = agent.get_tool(GetSymbolsOverviewTool)
    search_pattern = agent.get_tool(SearchForPatternTool)
    list_dir = agent.get_tool(ListDirTool)
    
    results = {}
    
    # 1. Analyze SerenaAgent class
    print("📊 ANALYZING: SerenaAgent class structure")
    print("-" * 80)
    result = agent.execute_task(
        lambda: find_symbol.apply(
            name_path="SerenaAgent",
            depth=1,
            relative_path="src/serena",
            include_body=False
        )
    )
    agent_symbols = json.loads(result)
    results['serena_agent'] = agent_symbols
    print(f"Found SerenaAgent with {len(agent_symbols)} symbol(s)")
    if agent_symbols:
        methods = [s for s in agent_symbols[0].get('children', []) if s.get('kind') == 6]
        print(f"  - Methods: {len(methods)}")
        print(f"  - Key methods: {', '.join([m['name_path'].split('/')[-1] for m in methods[:5]])}")
    print()
    
    # 2. Analyze Tool base class
    print("🔧 ANALYZING: Tool base class and inheritance")
    print("-" * 80)
    result = agent.execute_task(
        lambda: find_symbol.apply(
            name_path="Tool",
            depth=1,
            relative_path="src/serena/tools",
            include_body=False,
            include_kinds=[5]  # Classes only
        )
    )
    tool_classes = json.loads(result)
    results['tool_base'] = tool_classes
    print(f"Found Tool base class with {len(tool_classes)} definition(s)")
    print()
    
    # 3. Find all tool implementations
    print("🛠️  ANALYZING: Tool implementations (classes ending with 'Tool')")
    print("-" * 80)
    result = agent.execute_task(
        lambda: search_pattern.apply(
            substring_pattern="class.*Tool\\(",
            relative_path="src/serena/tools",
            paths_include_glob="*.py"
        )
    )
    tool_patterns = json.loads(result)
    results['tool_implementations'] = tool_patterns
    print(f"Found {len(tool_patterns.get('matches', []))} tool class definitions")
    tool_files = set(m['relative_path'] for m in tool_patterns.get('matches', []))
    print(f"  - Across {len(tool_files)} files: {', '.join(sorted(tool_files))}")
    print()
    
    # 4. Analyze LSP integration
    print("🔗 ANALYZING: SolidLanguageServer (LSP wrapper)")
    print("-" * 80)
    result = agent.execute_task(
        lambda: find_symbol.apply(
            name_path="SolidLanguageServer",
            depth=1,
            relative_path="src/solidlsp",
            include_body=False
        )
    )
    lsp_symbols = json.loads(result)
    results['solid_language_server'] = lsp_symbols
    if lsp_symbols:
        methods = [s for s in lsp_symbols[0].get('children', []) if s.get('kind') == 6]
        print(f"Found SolidLanguageServer with {len(methods)} methods")
        print(f"  - Key capabilities: {', '.join([m['name_path'].split('/')[-1] for m in methods[:8]])}")
    print()
    
    # 5. Find language server implementations
    print("🌐 ANALYZING: Language-specific server implementations")
    print("-" * 80)
    result = agent.execute_task(
        lambda: list_dir.apply(
            relative_path="src/solidlsp/language_servers",
            recursive=False,
            skip_ignored_files=True
        )
    )
    ls_files = json.loads(result)
    py_files = [f for f in ls_files.get('files', []) if f.endswith('.py') and not f.endswith('__init__.py')]
    results['language_servers'] = py_files
    print(f"Found {len(py_files)} language server implementations:")
    for f in sorted(py_files[:10]):
        print(f"  - {f}")
    if len(py_files) > 10:
        print(f"  ... and {len(py_files) - 10} more")
    print()
    
    # 6. Analyze configuration system
    print("⚙️  ANALYZING: Configuration classes")
    print("-" * 80)
    result = agent.execute_task(
        lambda: get_overview.apply(
            relative_path="src/serena/config/serena_config.py"
        )
    )
    config_symbols = json.loads(result)
    results['config_classes'] = config_symbols
    config_classes = [s for s in config_symbols if s.get('kind') == 5]
    print(f"Found {len(config_classes)} configuration classes:")
    for cls in config_classes[:10]:
        print(f"  - {cls['name_path']}")
    print()
    
    # 7. Analyze MCP server implementation
    print("📡 ANALYZING: MCP server factories")
    print("-" * 80)
    result = agent.execute_task(
        lambda: find_symbol.apply(
            name_path="MCP",
            depth=0,
            relative_path="src/serena/mcp.py",
            substring_matching=True,
            include_kinds=[5]  # Classes
        )
    )
    mcp_classes = json.loads(result)
    results['mcp_factories'] = mcp_classes
    print(f"Found {len(mcp_classes)} MCP-related classes:")
    for cls in mcp_classes:
        print(f"  - {cls['name_path']} (line {cls.get('body_location', {}).get('start_line', '?')})")
    print()
    
    # 8. Count integration examples
    print("📚 ANALYZING: Example scripts and documentation")
    print("-" * 80)
    result = agent.execute_task(
        lambda: list_dir.apply(
            relative_path="scripts",
            recursive=False,
            skip_ignored_files=True
        )
    )
    scripts = json.loads(result)
    py_scripts = [f for f in scripts.get('files', []) if f.endswith('.py')]
    results['example_scripts'] = py_scripts
    print(f"Found {len(py_scripts)} example Python scripts in /scripts:")
    for script in sorted(py_scripts):
        print(f"  - {script}")
    print()
    
    # 9. Find usage of SerenaAgent (library integration patterns)
    print("🔍 ANALYZING: SerenaAgent usage patterns (library integration)")
    print("-" * 80)
    result = agent.execute_task(
        lambda: search_pattern.apply(
            substring_pattern="SerenaAgent\\(",
            relative_path=".",
            paths_include_glob="*.py"
        )
    )
    usage_patterns = json.loads(result)
    results['agent_usage'] = usage_patterns
    usage_files = set(m['relative_path'] for m in usage_patterns.get('matches', []))
    print(f"Found {len(usage_patterns.get('matches', []))} instantiations of SerenaAgent:")
    for file in sorted(usage_files)[:10]:
        count = sum(1 for m in usage_patterns.get('matches', []) if m['relative_path'] == file)
        print(f"  - {file} ({count}x)")
    print()
    
    # 10. Summary statistics
    print("=" * 80)
    print("📊 SUMMARY STATISTICS")
    print("=" * 80)
    print(f"Tool implementations found: {len(tool_patterns.get('matches', []))}")
    print(f"Language servers supported: {len(py_files)}")
    print(f"Configuration classes: {len(config_classes)}")
    print(f"Example scripts: {len(py_scripts)}")
    print(f"SerenaAgent instantiation sites: {len(usage_patterns.get('matches', []))}")
    print()
    
    # Save results
    output_file = Path("serena_analysis_results.json")
    with open(output_file, 'w') as f:
        json.dump(results, f, indent=2)
    print(f"✅ Complete analysis saved to: {output_file}")
    print()
    
    return results

if __name__ == "__main__":
    try:
        results = analyze_serena()
        print("✨ Analysis complete! Serena successfully analyzed itself using its own tools.")
    except Exception as e:
        print(f"❌ Analysis failed: {e}")
        import traceback
        traceback.print_exc()
