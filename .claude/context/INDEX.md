# Serena Project Context Index

This directory contains project context files for AI agent coordination and session continuity.

## Latest Context

**Current Active Context:** [serena-rust-dashboard-indicator-2025-12-22.md](./serena-rust-dashboard-indicator-2025-12-22.md)

## Context Files

| File | Date | Project/Focus | Status |
|------|------|---------------|--------|
| `serena-rust-dashboard-indicator-2025-12-22.md` | 2025-12-22 | Dashboard Runtime Indicator + MCP | Active - Current Development |
| `serena-rust-mcp-implementation-2025-12-21.md` | 2025-12-21 | Rust MCP Server (16 tools) | Complete - Reference |
| `serena-rust-modernization-2025-12-21.md` | 2025-12-21 | Pure Rust Migration Planning | Superseded by implementation |

## Usage Instructions

### For Claude Code Sessions

1. **At session start:** Read the latest context file to understand project state
2. **During work:** Reference context for design decisions and patterns
3. **At session end:** Update context if significant progress was made

### For Agent Coordination

When delegating to specialized agents, include:
- Relevant section from context file
- Current phase in roadmap
- Specific task focus

### Context File Structure

Each context file contains:
1. **Project Overview** - Goals, architecture, technology stack
2. **Current State** - What's done, what's in progress
3. **Design Decisions** - Patterns, rationale, code examples
4. **Agent History** - Previous agent sessions and outcomes
5. **Migration Roadmap** - Timeline and milestones
6. **Expected Outcomes** - Performance targets, deliverables
7. **Risk Assessment** - Technical and migration risks
8. **Recovery Instructions** - How to restore context
9. **Quick Reference** - Commands, directories, agents

## Versioning Strategy

- **Major updates:** New date-stamped file (e.g., `project-2025-12-25.md`)
- **Minor updates:** Edit existing file, update "Last Updated" timestamp
- **Archives:** Move completed phase contexts to `archive/` subdirectory

## Related Documentation

Root-level documentation files:
- `SERENA_RUST_MODERNIZATION_PLAN.md` - Main planning document
- `RUST_MCP_RESEARCH.md` - MCP SDK research
- `RUST_MIGRATION_ARCHITECTURE.md` - Architecture design
- `RUST_MIGRATION_ANALYSIS.md` - Python dependency analysis
- `RUST_MCP_ARCHITECTURE.md` - MCP implementation details
- `RUST_MIGRATION_ROADMAP.md` - Detailed timeline
- `RUST_MIGRATION_SUMMARY.md` - Executive summary

---

*Maintained by Claude AI Context Manager*
