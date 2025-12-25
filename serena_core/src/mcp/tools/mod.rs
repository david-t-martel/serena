//! Serena Tool Implementations
//!
//! This module contains all the MCP tools exposed by the Serena server.

pub mod cli;
pub mod file_tools;
pub mod symbol_tools;
pub mod memory_tools;
pub mod services;

pub use file_tools::*;
pub use symbol_tools::*;
pub use memory_tools::*;
pub use services::*;

use rust_mcp_sdk::schema::Tool;

/// All Serena tools enumeration - used for routing and listing
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "name", content = "arguments")]
pub enum SerenaTools {
    // File Tools
    #[serde(rename = "read_file")]
    ReadFile(ReadFileTool),
    #[serde(rename = "create_text_file")]
    CreateTextFile(CreateTextFileTool),
    #[serde(rename = "list_dir")]
    ListDir(ListDirTool),
    #[serde(rename = "find_file")]
    FindFile(FindFileTool),
    #[serde(rename = "replace_content")]
    ReplaceContent(ReplaceContentTool),
    #[serde(rename = "search_for_pattern")]
    SearchForPattern(SearchForPatternTool),

    // Symbol Tools
    #[serde(rename = "get_symbols_overview")]
    GetSymbolsOverview(GetSymbolsOverviewTool),
    #[serde(rename = "find_symbol")]
    FindSymbol(FindSymbolTool),
    #[serde(rename = "find_referencing_symbols")]
    FindReferencingSymbols(FindReferencingSymbolsTool),
    #[serde(rename = "replace_symbol_body")]
    ReplaceSymbolBody(ReplaceSymbolBodyTool),
    #[serde(rename = "rename_symbol")]
    RenameSymbol(RenameSymbolTool),

    // Memory Tools
    #[serde(rename = "write_memory")]
    WriteMemory(WriteMemoryTool),
    #[serde(rename = "read_memory")]
    ReadMemory(ReadMemoryTool),
    #[serde(rename = "list_memories")]
    ListMemories(ListMemoriesTool),
    #[serde(rename = "delete_memory")]
    DeleteMemory(DeleteMemoryTool),
    #[serde(rename = "edit_memory")]
    EditMemory(EditMemoryTool),
}

impl SerenaTools {
    /// Get all tool definitions for MCP listing
    pub fn all_tools() -> Vec<Tool> {
        vec![
            // File Tools
            ReadFileTool::tool(),
            CreateTextFileTool::tool(),
            ListDirTool::tool(),
            FindFileTool::tool(),
            ReplaceContentTool::tool(),
            SearchForPatternTool::tool(),

            // Symbol Tools
            GetSymbolsOverviewTool::tool(),
            FindSymbolTool::tool(),
            FindReferencingSymbolsTool::tool(),
            ReplaceSymbolBodyTool::tool(),
            RenameSymbolTool::tool(),

            // Memory Tools
            WriteMemoryTool::tool(),
            ReadMemoryTool::tool(),
            ListMemoriesTool::tool(),
            DeleteMemoryTool::tool(),
            EditMemoryTool::tool(),
        ]
    }
}

impl TryFrom<rust_mcp_sdk::schema::CallToolRequestParams> for SerenaTools {
    type Error = String;

    fn try_from(params: rust_mcp_sdk::schema::CallToolRequestParams) -> Result<Self, Self::Error> {
        let name = params.name.as_str();
        let args = serde_json::Value::Object(params.arguments.unwrap_or_default());

        match name {
            // File Tools
            "read_file" => serde_json::from_value(args)
                .map(SerenaTools::ReadFile)
                .map_err(|e| e.to_string()),
            "create_text_file" => serde_json::from_value(args)
                .map(SerenaTools::CreateTextFile)
                .map_err(|e| e.to_string()),
            "list_dir" => serde_json::from_value(args)
                .map(SerenaTools::ListDir)
                .map_err(|e| e.to_string()),
            "find_file" => serde_json::from_value(args)
                .map(SerenaTools::FindFile)
                .map_err(|e| e.to_string()),
            "replace_content" => serde_json::from_value(args)
                .map(SerenaTools::ReplaceContent)
                .map_err(|e| e.to_string()),
            "search_for_pattern" => serde_json::from_value(args)
                .map(SerenaTools::SearchForPattern)
                .map_err(|e| e.to_string()),

            // Symbol Tools
            "get_symbols_overview" => serde_json::from_value(args)
                .map(SerenaTools::GetSymbolsOverview)
                .map_err(|e| e.to_string()),
            "find_symbol" => serde_json::from_value(args)
                .map(SerenaTools::FindSymbol)
                .map_err(|e| e.to_string()),
            "find_referencing_symbols" => serde_json::from_value(args)
                .map(SerenaTools::FindReferencingSymbols)
                .map_err(|e| e.to_string()),
            "replace_symbol_body" => serde_json::from_value(args)
                .map(SerenaTools::ReplaceSymbolBody)
                .map_err(|e| e.to_string()),
            "rename_symbol" => serde_json::from_value(args)
                .map(SerenaTools::RenameSymbol)
                .map_err(|e| e.to_string()),

            // Memory Tools
            "write_memory" => serde_json::from_value(args)
                .map(SerenaTools::WriteMemory)
                .map_err(|e| e.to_string()),
            "read_memory" => serde_json::from_value(args)
                .map(SerenaTools::ReadMemory)
                .map_err(|e| e.to_string()),
            "list_memories" => serde_json::from_value(args)
                .map(SerenaTools::ListMemories)
                .map_err(|e| e.to_string()),
            "delete_memory" => serde_json::from_value(args)
                .map(SerenaTools::DeleteMemory)
                .map_err(|e| e.to_string()),
            "edit_memory" => serde_json::from_value(args)
                .map(SerenaTools::EditMemory)
                .map_err(|e| e.to_string()),

            _ => Err(format!("Unknown tool: {}", name)),
        }
    }
}
