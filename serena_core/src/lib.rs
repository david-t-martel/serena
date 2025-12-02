use anyhow::Result;
use ignore::WalkBuilder;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rayon::prelude::*;
use regex::RegexBuilder;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct MatchLine {
    line_number: usize,
    content: String,
    match_type: &'static str,
}

#[derive(Debug)]
struct FileMatch {
    lines: Vec<MatchLine>,
}

fn search_files_impl(
    pattern: &str,
    root: &str,
    relative_paths: Vec<String>,
    context_lines_before: usize,
    context_lines_after: usize,
) -> Result<Vec<(String, Vec<FileMatch>)>> {
    let re = RegexBuilder::new(pattern)
        .dot_matches_new_line(true)
        .build()
        .map_err(|e| anyhow::anyhow!("Invalid regex pattern: {e}"))?;

    let root_path = PathBuf::from(root);

    // Parallelise across files using rayon. For each file we read the contents
    // and compute FileMatch structures; unreadable files are skipped, mirroring
    // the Python implementation.
    let results: Vec<(String, Vec<FileMatch>)> = relative_paths
        .into_par_iter()
        .filter_map(|rel_path| {
            let full_path = root_path.join(&rel_path);
            let content = match fs::read_to_string(&full_path) {
                Ok(c) => c,
                Err(_) => return None,
            };

            let file_matches = match search_in_content(
                &content,
                &re,
                context_lines_before,
                context_lines_after,
            ) {
                Ok(m) if !m.is_empty() => m,
                _ => return None,
            };

            Some((rel_path, file_matches))
        })
        .collect();

    Ok(results)
}

#[pyfunction]
fn search_files(
    py: Python<'_>,
    pattern: &str,
    root: &str,
    relative_paths: Vec<String>,
    context_lines_before: usize,
    context_lines_after: usize,
) -> PyResult<Vec<PyObject>> {
    // Run the heavy I/O and regex work without holding the GIL.
    let raw_results = py
        .allow_threads(|| {
            search_files_impl(
                pattern,
                root,
                relative_paths,
                context_lines_before,
                context_lines_after,
            )
        })
        .map_err(|e| PyValueError::new_err(format!("search_files failed: {e}")))?;

    let mut out: Vec<PyObject> = Vec::with_capacity(raw_results.len());

    for (rel_path, file_matches) in raw_results {
        for m in file_matches {
            let dict = PyDict::new(py);
            dict.set_item("path", &rel_path)?;

            let mut lines_objs: Vec<PyObject> = Vec::with_capacity(m.lines.len());
            for line in m.lines {
                let line_dict = PyDict::new(py);
                line_dict.set_item("line_number", line.line_number)?;
                line_dict.set_item("content", line.content)?;
                line_dict.set_item("match_type", line.match_type)?; // "match", "prefix", or "postfix"
                lines_objs.push(line_dict.into_py(py));
            }

            dict.set_item("lines", lines_objs)?;
            out.push(dict.into_py(py));
        }
    }

    Ok(out)
}

#[pyfunction]
fn walk_files_gitignored(root: &str, start: Option<&str>) -> PyResult<Vec<String>> {
    // Root of the project
    let root_path = PathBuf::from(root);
    let start_path = match start {
        Some(rel) if !rel.is_empty() => root_path.join(rel),
        _ => root_path.clone(),
    };

    let mut builder = WalkBuilder::new(&start_path);
    builder
        .git_ignore(true)
        .git_exclude(true)
        .git_global(true)
        .hidden(false)
        .follow_links(true);

    let walker = builder.build();
    let mut results = Vec::new();

    for dent in walker {
        let entry = match dent {
            Ok(e) => e,
            Err(_) => continue,
        };
        // Only include regular files
        let md = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if !md.is_file() {
            continue;
        }

        let path = entry.path();
        // Make path relative to the project root (not the start path)
        let rel = match path.strip_prefix(&root_path) {
            Ok(r) => r,
            Err(_) => continue,
        };
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        results.push(rel_str);
    }

    Ok(results)
}

fn search_in_content(
    content: &str,
    re: &regex::Regex,
    context_before: usize,
    context_after: usize,
) -> Result<Vec<FileMatch>> {
    let mut matches = Vec::new();

    // Precompute line start offsets for fast position->line lookup (based on '\n').
    // This is used only for offset->line mapping; the total number of lines is
    // derived from content.lines() to mirror Python's splitlines() behaviour.
    let mut line_starts = Vec::new();
    line_starts.push(0usize);
    for (idx, ch) in content.char_indices() {
        if ch == '\n' {
            line_starts.push(idx + 1);
        }
    }

    // Helper: map byte offset to 1-based line number (similar to Python's
    // `content[:offset].count("\n") + 1`).
    let offset_to_line = |offset: usize, line_starts: &Vec<usize>| -> usize {
        match line_starts.binary_search(&offset) {
            Ok(i) => i + 1,
            Err(i) => {
                if i == 0 {
                    1
                } else {
                    i
                }
            }
        }
    };

    // Use content.lines() rather than split('\n') so that total_lines matches
    // Python's splitlines() semantics for trailing newlines.
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();

    for m in re.find_iter(content) {
        let start = m.start();
        let end = m.end();

        let start_line_num = offset_to_line(start, &line_starts);
        let end_line_num = offset_to_line(end, &line_starts);

        let context_start = if start_line_num > context_before {
            start_line_num.saturating_sub(context_before)
        } else {
            1
        };
        let mut context_end = end_line_num + context_after;
        if context_end > total_lines {
            context_end = total_lines;
        }

        let mut out_lines = Vec::new();
        for line_num in context_start..=context_end {
            let idx = line_num - 1; // lines are 1-based here
            if idx >= lines.len() {
                break;
            }
            let line_content = lines[idx].to_string();
            let match_type = if line_num < start_line_num {
                "prefix"
            } else if line_num > end_line_num {
                "postfix"
            } else {
                "match"
            };
            out_lines.push(MatchLine {
                line_number: line_num,
                content: line_content,
                match_type,
            });
        }

        if !out_lines.is_empty() {
            matches.push(FileMatch { lines: out_lines });
        }
    }

    Ok(matches)
}

#[pymodule]
fn serena_core(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(search_files, m)?)?;
    m.add_function(wrap_pyfunction!(walk_files_gitignored, m)?)?;
    Ok(())
}
