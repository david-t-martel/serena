# Serena Rust MCP Server - Quick Context Load

**Use this for fast session restoration. For full details, see the dated context file.**

## TL;DR

Rust MCP server implementation COMPLETE. 16 tools using rust-mcp-sdk v0.7. Binary: 3.9MB. Dashboard now shows Rust/Python runtime indicator badge with Rust-orange (#F74C00) branding.

## Current Phase

**Phase: Dashboard Enhancement Complete** (as of 2025-12-22)
- DONE: Runtime indicator badge showing Rust/Python backend status
- Next: Test with live Rust server, add config tools

## Recent Session (2025-12-22)

Added dashboard runtime indicator badge:
- **Files:** `serena_core/src/web/mod.rs`, dashboard HTML/CSS/JS
- **States:** Connecting (pulse), Rust Active (orange), Python (blue), Error (red)
- **Heartbeat:** `{"status":"ok","agent":"serena-rust","version":"X.X.X"}`

## Key Decisions Made

1. **MCP SDK:** rust-mcp-sdk v0.7 (NOT rmcp v0.9.0 from planning)
2. **Architecture:** Single serena_core crate (not 9-crate workspace)
3. **Storage:** Markdown files for memory (not SQLite)
4. **Pattern:** Service layer (FileService, SymbolService, MemoryService)
5. **UI Colors:** Rust #F74C00, Python #3776AB, Error #DC3545
6. **CSS:** BEM naming (runtime-badge, runtime-badge__icon, runtime-badge--rust)

## Implemented Tools (16 Total)

**File Tools (6):** read_file, create_text_file, list_dir, find_file, replace_content, search_for_pattern

**Symbol Tools (5):** get_symbols_overview, find_symbol, find_referencing_symbols, replace_symbol_body, rename_symbol

**Memory Tools (5):** write_memory, read_memory, list_memories, delete_memory, edit_memory

## Key Files

```
# Rust MCP Implementation
serena_core/src/mcp/mod.rs           # Module exports
serena_core/src/mcp/handler.rs       # ServerHandler impl
serena_core/src/mcp/tools/mod.rs     # SerenaTools enum
serena_core/src/mcp/tools/services.rs # Business logic
serena_core/src/web/mod.rs           # Dashboard backend (heartbeat endpoint)
serena_core/src/bin/mcp_server.rs    # Entry point

# Dashboard Frontend
src/serena/resources/dashboard/index.html     # Badge HTML (lines 32-36)
src/serena/resources/dashboard/dashboard.css  # Badge styles (lines 744-875)
src/serena/resources/dashboard/dashboard.js   # Detection logic (lines 416-468)
```

## Critical API Patterns (rust-mcp-sdk v0.7)

```rust
// Result construction
CallToolResult::text_content(vec![TextContent::from(s)])

// Error creation
CallToolError::from_message("Error message".to_string())

// Parameter parsing
let args = params.arguments.unwrap_or_default();
let tool_params: MyParams = serde_json::from_value(
    serde_json::Value::Object(args)
)?;
```

## Essential Commands

```bash
# Build release binary
cargo build --release -p serena_core --bin serena-mcp-server

# Binary location
T:\projects\serena-source\target\release\serena-mcp-server.exe

# Run with cargo
cargo run -p serena_core --bin serena-mcp-server

# Python tests
uv run poe test
uv run poe format
uv run poe type-check
```

## Agent Recommendations

| Task | Agent |
|------|-------|
| Rust implementation | rust-pro |
| Frontend CSS/JS | frontend-design |
| LSP integration | debugger |
| Performance tuning | performance-engineer |
| Architecture | backend-architect |

## Full Context

See: `serena-rust-dashboard-indicator-2025-12-22.md`

---

*Quick load version: 2025-12-22*
