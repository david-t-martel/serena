pub mod error;
pub mod types;
pub mod traits;
pub mod registry;

// Re-export error types
pub use error::{ConfigError, LspError, SerenaError, ToolError};

// Re-export core types
pub use types::{Location, Position, Range, SymbolInfo, SymbolKind, ToolResult, ToolStatus};

// Re-export traits
pub use traits::{LanguageServer, MemoryStorage, Tool};

// Re-export registry
pub use registry::{ToolRegistry, ToolRegistryBuilder};
