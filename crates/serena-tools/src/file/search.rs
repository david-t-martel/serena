use async_trait::async_trait;
use ignore::WalkBuilder;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serena_core::{SerenaError, Tool, ToolResult};
use std::path::PathBuf;
use tracing::debug;

/// Tool for searching files with regex patterns
pub struct SearchFilesTool {
    project_root: PathBuf,
}

#[derive(Debug, Deserialize)]
struct SearchFilesParams {
    pattern: String,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    include_glob: Option<String>,
    #[serde(default)]
    exclude_glob: Option<String>,
    #[serde(default)]
    case_insensitive: Option<bool>,
    #[serde(default)]
    max_results: Option<usize>,
    #[serde(default)]
    context_lines: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileMatch {
    path: String,
    line_number: usize,
    line: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    context_before: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context_after: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchFilesOutput {
    matches: Vec<FileMatch>,
    total_matches: usize,
    truncated: bool,
}

impl SearchFilesTool {
    /// Create a new SearchFilesTool with the given project root
    pub fn new(project_root: impl Into<PathBuf>) -> Self {
        Self {
            project_root: project_root.into(),
        }
    }

    async fn search_impl(
        &self,
        params: SearchFilesParams,
    ) -> Result<SearchFilesOutput, SerenaError> {
        // Build regex pattern
        let regex = RegexBuilder::new(&params.pattern)
            .case_insensitive(params.case_insensitive.unwrap_or(false))
            .build()
            .map_err(|e| SerenaError::InvalidParameter(
                format!("Invalid regex pattern: {}", e)
            ))?;

        // Determine search root
        let search_root = if let Some(ref path) = params.path {
            self.project_root.join(path)
        } else {
            self.project_root.clone()
        };

        debug!("Searching in: {:?} with pattern: {}", search_root, params.pattern);

        // Build file walker
        let mut builder = WalkBuilder::new(&search_root);
        builder
            .git_ignore(true)
            .git_exclude(true)
            .git_global(true)
            .hidden(false)
            .follow_links(true);

        let walker = builder.build();
        let max_results = params.max_results.unwrap_or(1000);
        let context_lines = params.context_lines.unwrap_or(0);

        let mut all_matches = Vec::new();
        let mut total_count = 0;

        // Walk and search files
        for entry in walker {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            // Only process files
            if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                continue;
            }

            let path = entry.path();

            // Apply glob filters if specified
            if let Some(ref include) = params.include_glob {
                if !glob_match(path, include) {
                    continue;
                }
            }

            if let Some(ref exclude) = params.exclude_glob {
                if glob_match(path, exclude) {
                    continue;
                }
            }

            // Read and search file
            if let Ok(content) = std::fs::read_to_string(path) {
                let lines: Vec<&str> = content.lines().collect();

                for (idx, line) in lines.iter().enumerate() {
                    if regex.is_match(line) {
                        total_count += 1;

                        if all_matches.len() < max_results {
                            let relative_path = path.strip_prefix(&self.project_root)
                                .unwrap_or(path)
                                .to_string_lossy()
                                .replace('\\', "/");

                            // Collect context lines if requested
                            let context_before = if context_lines > 0 {
                                let start = idx.saturating_sub(context_lines);
                                Some(lines[start..idx].iter().map(|s| s.to_string()).collect())
                            } else {
                                None
                            };

                            let context_after = if context_lines > 0 {
                                let end = (idx + 1 + context_lines).min(lines.len());
                                Some(lines[idx + 1..end].iter().map(|s| s.to_string()).collect())
                            } else {
                                None
                            };

                            all_matches.push(FileMatch {
                                path: relative_path,
                                line_number: idx + 1, // 1-based line numbers
                                line: line.to_string(),
                                context_before,
                                context_after,
                            });
                        }
                    }
                }
            }
        }

        Ok(SearchFilesOutput {
            matches: all_matches,
            total_matches: total_count,
            truncated: total_count > max_results,
        })
    }
}

/// Simple glob pattern matching
fn glob_match(path: &std::path::Path, pattern: &str) -> bool {
    glob::Pattern::new(pattern)
        .ok()
        .and_then(|p| Some(p.matches_path(path)))
        .unwrap_or(false)
}

#[async_trait]
impl Tool for SearchFilesTool {
    fn name(&self) -> &str {
        "search_files"
    }

    fn description(&self) -> &str {
        "Search for text patterns in files using regex. Respects .gitignore. \
        Supports glob patterns for filtering files."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "Regex pattern to search for"
                },
                "path": {
                    "type": "string",
                    "description": "Optional relative path to search within (defaults to project root)"
                },
                "include_glob": {
                    "type": "string",
                    "description": "Optional glob pattern to include files (e.g., '*.rs')"
                },
                "exclude_glob": {
                    "type": "string",
                    "description": "Optional glob pattern to exclude files (e.g., '*.test.rs')"
                },
                "case_insensitive": {
                    "type": "boolean",
                    "description": "Whether to perform case-insensitive search (default: false)"
                },
                "max_results": {
                    "type": "integer",
                    "description": "Maximum number of results to return (default: 1000)"
                },
                "context_lines": {
                    "type": "integer",
                    "description": "Number of context lines to include before/after match (default: 0)"
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult, SerenaError> {
        let params: SearchFilesParams = serde_json::from_value(params)
            .map_err(|e| SerenaError::InvalidParameter(e.to_string()))?;

        let output = self.search_impl(params).await?;

        let message = if output.truncated {
            format!(
                "Found {} matches (showing first {})",
                output.total_matches,
                output.matches.len()
            )
        } else {
            format!("Found {} matches", output.total_matches)
        };

        Ok(ToolResult::success_with_message(
            serde_json::to_value(output)?,
            message
        ))
    }

    fn can_edit(&self) -> bool {
        false
    }

    fn requires_project(&self) -> bool {
        true
    }

    fn tags(&self) -> Vec<String> {
        vec!["file".to_string(), "search".to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::fs::write;

    #[tokio::test]
    async fn test_basic_search() {
        let temp_dir = TempDir::new().unwrap();
        write(temp_dir.path().join("file1.txt"), "Hello World\nGoodbye World").await.unwrap();
        write(temp_dir.path().join("file2.txt"), "Hello Rust\nGoodbye Rust").await.unwrap();

        let tool = SearchFilesTool::new(temp_dir.path());

        let params = json!({
            "pattern": "Hello"
        });

        let result = tool.execute(params).await.unwrap();
        let data = result.data.unwrap();
        let output: SearchFilesOutput = serde_json::from_value(data).unwrap();

        assert_eq!(output.total_matches, 2);
        assert_eq!(output.matches.len(), 2);
    }

    #[tokio::test]
    async fn test_case_insensitive_search() {
        let temp_dir = TempDir::new().unwrap();
        write(temp_dir.path().join("test.txt"), "hello\nHELLO\nHeLLo").await.unwrap();

        let tool = SearchFilesTool::new(temp_dir.path());

        let params = json!({
            "pattern": "hello",
            "case_insensitive": true
        });

        let result = tool.execute(params).await.unwrap();
        let data = result.data.unwrap();
        let output: SearchFilesOutput = serde_json::from_value(data).unwrap();

        assert_eq!(output.total_matches, 3);
    }

    #[tokio::test]
    async fn test_context_lines() {
        let temp_dir = TempDir::new().unwrap();
        write(temp_dir.path().join("test.txt"), "Line 1\nLine 2\nTarget\nLine 4\nLine 5").await.unwrap();

        let tool = SearchFilesTool::new(temp_dir.path());

        let params = json!({
            "pattern": "Target",
            "context_lines": 1
        });

        let result = tool.execute(params).await.unwrap();
        let data = result.data.unwrap();
        let output: SearchFilesOutput = serde_json::from_value(data).unwrap();

        assert_eq!(output.matches.len(), 1);
        let match_result = &output.matches[0];
        assert_eq!(match_result.context_before.as_ref().unwrap().len(), 1);
        assert_eq!(match_result.context_after.as_ref().unwrap().len(), 1);
    }
}
