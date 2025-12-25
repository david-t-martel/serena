use std::collections::HashMap;
use serena_core::Tool;

/// Registry for managing tool instances
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    /// Create a new empty tool registry
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Register a tool in the registry
    pub fn register(&mut self, tool: Box<dyn Tool>) {
        let name = tool.name().to_string();
        self.tools.insert(name, tool);
    }

    /// Get a tool by name
    pub fn get(&self, name: &str) -> Option<&dyn Tool> {
        self.tools.get(name).map(|t| t.as_ref())
    }

    /// List all registered tool names
    pub fn list(&self) -> Vec<&str> {
        self.tools.keys().map(|s| s.as_str()).collect()
    }

    /// Get the number of registered tools
    pub fn len(&self) -> usize {
        self.tools.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    /// Remove a tool from the registry
    pub fn remove(&mut self, name: &str) -> Option<Box<dyn Tool>> {
        self.tools.remove(name)
    }

    /// Clear all tools from the registry
    pub fn clear(&mut self) {
        self.tools.clear();
    }

    /// Check if a tool is registered
    pub fn contains(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }

    /// Get all tools matching a tag
    pub fn get_by_tag(&self, tag: &str) -> Vec<&dyn Tool> {
        self.tools
            .values()
            .filter(|t| t.tags().iter().any(|t_tag| t_tag == tag))
            .map(|t| t.as_ref())
            .collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use serde_json::Value;
    use serena_core::{ToolResult, SerenaError};

    struct MockTool {
        name: String,
        tags: Vec<String>,
    }

    #[async_trait]
    impl Tool for MockTool {
        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            "A mock tool"
        }

        fn parameters_schema(&self) -> Value {
            serde_json::json!({})
        }

        async fn execute(&self, _params: Value) -> Result<ToolResult, SerenaError> {
            Ok(ToolResult::success(serde_json::json!({})))
        }

        fn tags(&self) -> Vec<String> {
            self.tags.clone()
        }
    }

    #[test]
    fn test_registry_new() {
        let registry = ToolRegistry::new();
        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());
    }

    #[test]
    fn test_register_and_get() {
        let mut registry = ToolRegistry::new();
        let tool = Box::new(MockTool {
            name: "test_tool".to_string(),
            tags: vec![],
        });
        registry.register(tool);

        assert_eq!(registry.len(), 1);
        assert!(registry.contains("test_tool"));
        assert!(registry.get("test_tool").is_some());
    }

    #[test]
    fn test_list() {
        let mut registry = ToolRegistry::new();
        registry.register(Box::new(MockTool {
            name: "tool1".to_string(),
            tags: vec![],
        }));
        registry.register(Box::new(MockTool {
            name: "tool2".to_string(),
            tags: vec![],
        }));

        let names = registry.list();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"tool1"));
        assert!(names.contains(&"tool2"));
    }

    #[test]
    fn test_get_by_tag() {
        let mut registry = ToolRegistry::new();
        registry.register(Box::new(MockTool {
            name: "tool1".to_string(),
            tags: vec!["file".to_string()],
        }));
        registry.register(Box::new(MockTool {
            name: "tool2".to_string(),
            tags: vec!["symbol".to_string()],
        }));
        registry.register(Box::new(MockTool {
            name: "tool3".to_string(),
            tags: vec!["file".to_string(), "search".to_string()],
        }));

        let file_tools = registry.get_by_tag("file");
        assert_eq!(file_tools.len(), 2);
    }
}
