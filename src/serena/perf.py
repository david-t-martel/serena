"""Performance comparison helpers for Serena.

This module provides utilities to compare Python-only and Rust-accelerated
implementations of core operations such as source file gathering and
multi-file search. It is primarily intended for local benchmarking and
profiling, not for production use inside tools.
"""

from __future__ import annotations

import logging
import time
from dataclasses import dataclass
from typing import Any

import psutil

from serena import text_utils
from serena.project import Project

log = logging.getLogger(__name__)


@dataclass
class TimingResult:
    backend: str
    seconds: float
    runs: int
    extra: dict[str, Any] | None = None


def benchmark_gather_backends(
    project_root: str,
    relative_path: str = "",
    runs: int = 3,
) -> dict[str, TimingResult]:
    """Benchmark Project.gather_source_files using Python and Rust backends.

    This assumes Project.gather_source_files prefers Rust when available.
    We explicitly call the Python-only and Rust-only helpers for a fair
    comparison.
    """
    project = Project.load(project_root)

    # Expect helper methods to exist; if not, fall back to the public method.
    python_func = getattr(project, "_gather_source_files_python", project.gather_source_files)
    rust_func = getattr(project, "_gather_source_files_rust", None)

    results: dict[str, TimingResult] = {}

    # Warm-up
    python_func(relative_path)
    if rust_func is not None:
        rust_func(relative_path)

    proc = psutil.Process()

    # Python baseline
    t0 = time.perf_counter()
    for _ in range(runs):
        files_py = python_func(relative_path)
    t1 = time.perf_counter()
    mem_py = proc.memory_info().rss
    results["python"] = TimingResult(
        backend="python",
        seconds=(t1 - t0) / runs,
        runs=runs,
        extra={"num_files": len(files_py), "rss_bytes": mem_py},
    )

    # Rust path (if available)
    if rust_func is not None:
        t0 = time.perf_counter()
        for _ in range(runs):
            files_rust = rust_func(relative_path)
        t1 = time.perf_counter()
        mem_rust = proc.memory_info().rss
        results["rust"] = TimingResult(
            backend="rust",
            seconds=(t1 - t0) / runs,
            runs=runs,
            extra={"num_files": len(files_rust), "rss_bytes": mem_rust},
        )

    return results


def benchmark_search_backends(
    project_root: str,
    pattern: str,
    relative_path: str = "",
    context_lines_before: int = 0,
    context_lines_after: int = 0,
    paths_include_glob: str | None = None,
    paths_exclude_glob: str | None = None,
    runs: int = 3,
) -> dict[str, TimingResult]:
    """Benchmark multi-file search in Python vs Rust backends.

    This uses the internal helpers in serena.text_utils so we can measure
    the backends independently of any environment variables.
    """
    project = Project.load(project_root)
    rel_paths = project.gather_source_files(relative_path=relative_path)

    python_func = text_utils._search_files_python  # type: ignore[attr-defined]
    rust_func = getattr(text_utils, "_search_files_rust", None)

    # Warm-up
    _ = python_func(
        rel_paths,
        pattern,
        root_path=project.project_root,
        file_reader=project.read_file,
        context_lines_before=context_lines_before,
        context_lines_after=context_lines_after,
    )
    if rust_func is not None:
        _ = rust_func(
            rel_paths,
            pattern,
            root_path=project.project_root,
            context_lines_before=context_lines_before,
            context_lines_after=context_lines_after,
        )

    results: dict[str, TimingResult] = {}

    proc = psutil.Process()

    # Python baseline
    t0 = time.perf_counter()
    last_py = []
    for _ in range(runs):
        last_py = python_func(
            rel_paths,
            pattern,
            root_path=project.project_root,
            file_reader=project.read_file,
            context_lines_before=context_lines_before,
            context_lines_after=context_lines_after,
        )
    t1 = time.perf_counter()
    mem_py = proc.memory_info().rss
    results["python"] = TimingResult(
        backend="python",
        seconds=(t1 - t0) / runs,
        runs=runs,
        extra={"num_matches": len(last_py), "num_files": len(rel_paths), "rss_bytes": mem_py},
    )

    # Rust path (if available)
    if rust_func is not None:
        t0 = time.perf_counter()
        last_rust = []
        for _ in range(runs):
            last_rust = rust_func(
                rel_paths,
                pattern,
                root_path=project.project_root,
                context_lines_before=context_lines_before,
                context_lines_after=context_lines_after,
            )
        t1 = time.perf_counter()
        mem_rust = proc.memory_info().rss
        results["rust"] = TimingResult(
            backend="rust",
            seconds=(t1 - t0) / runs,
            runs=runs,
            extra={"num_matches": len(last_rust), "num_files": len(rel_paths), "rss_bytes": mem_rust},
        )

    return results
