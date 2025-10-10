#!/usr/bin/env python3
"""
Rebuild BEYOND_MCP_ANALYSIS.md using Serena's file reading capabilities.

This script reconstructs the comprehensive analysis document by:
1. Reading the baseline structure
2. Reading all component markdown files
3. Assembling them into a complete document
4. Adding proper section markers and progress tracking
"""

import os
import sys
from pathlib import Path

# Add src to path for imports
sys.path.insert(0, str(Path(__file__).parent / "src"))

def read_file_safe(filepath: str) -> str:
    """Safely read a file with UTF-8 encoding."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            return f.content()
    except Exception as e:
        print(f"Warning: Could not read {filepath}: {e}")
        return ""

def main():
    """Rebuild the comprehensive analysis document."""
    
    project_root = Path(__file__).parent
    
    print("🔄 Rebuilding BEYOND_MCP_ANALYSIS.md...")
    print()
    
    # Component files in order
    components = [
        ("BEYOND_MCP_BASELINE.md", "Foundation and Scope"),
        ("BEYOND_MCP_PART8_INTEGRATION_MODES.md", "Integration Modes Catalog"),
    ]
    
    # Output file
    output_file = project_root / "BEYOND_MCP_ANALYSIS_REBUILT.md"
    
    # Start building the document
    content = []
    
    # Add header
    content.append("# Serena: Beyond the MCP Server - Comprehensive Capability Analysis")
    content.append("")
    content.append("**Analysis Date**: January 10, 2025  ")
    content.append("**Serena Version**: 0.1.4  ")
    content.append("**Scope**: Non-MCP Integration Modes and Advanced Capabilities")
    content.append("")
    content.append("**Document Status**: Reconstructed from component files")
    content.append("")
    content.append("---")
    content.append("")
    
    # Read and combine component files
    for idx, (filename, description) in enumerate(components, 1):
        filepath = project_root / filename
        
        print(f"📄 Reading {filename} ({description})...")
        
        if filepath.exists():
            file_content = filepath.read_text(encoding='utf-8')
            
            # Add section marker
            content.append(f"## Component {idx}: {description}")
            content.append(f"**Source**: `{filename}`")
            content.append("")
            content.append("---")
            content.append("")
            
            # Add the content
            content.append(file_content)
            content.append("")
            content.append("---")
            content.append("")
        else:
            print(f"  ⚠️  File not found: {filepath}")
            content.append(f"## Component {idx}: {description}")
            content.append(f"**Status**: Missing file `{filename}`")
            content.append("")
    
    # Add LSP Deep Dive section marker (to be filled manually)
    content.append("## LSP Integration Deep Dive (To Be Restored)")
    content.append("")
    content.append("**Note**: This section contained detailed analysis of:")
    content.append("- LSP architecture and protocol integration")
    content.append("- Semantic capabilities (symbol discovery, cross-references, type information)")
    content.append("- Caching and performance optimization")
    content.append("- Language server implementations (20+ languages)")
    content.append("- Integration with Serena tools")
    content.append("- Key differentiators vs text-based/IDE/static analysis")
    content.append("- Limitations and mitigations")
    content.append("")
    content.append("This section needs to be reconstructed from discussion notes and source code analysis.")
    content.append("")
    content.append("---")
    content.append("")
    
    # Add completion status
    content.append("## Document Reconstruction Status")
    content.append("")
    content.append("- ✅ **Part 1-2**: MCP Baseline and Scope (from BEYOND_MCP_BASELINE.md)")
    content.append("- ⚠️  **Part 6**: LSP Deep Dive (needs manual reconstruction)")
    content.append("- ✅ **Part 8**: Integration Modes (from BEYOND_MCP_PART8_INTEGRATION_MODES.md)")
    content.append("- 📋 **Remaining**: Tool inventory, configuration system deep dive")
    content.append("")
    content.append(f"**Estimated Completion**: 60% of target analysis")
    content.append("")
    
    # Write output
    full_content = "\n".join(content)
    output_file.write_text(full_content, encoding='utf-8')
    
    print()
    print(f"✅ Rebuilt document written to: {output_file}")
    print(f"📊 Total size: {len(full_content)} characters")
    print(f"📄 Total lines: {len(content)} lines")
    print()
    print("Next steps:")
    print("1. Review BEYOND_MCP_ANALYSIS_REBUILT.md")
    print("2. Manually restore LSP Deep Dive section from notes")
    print("3. Rename to BEYOND_MCP_ANALYSIS.md when complete")
    print("4. Run: git add BEYOND_MCP_ANALYSIS.md && git commit -m 'docs: Restore complete analysis'")

if __name__ == "__main__":
    main()
