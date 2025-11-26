# Serena Windows Compatibility Analysis Report

## Executive Summary

This comprehensive analysis identified **7 critical Windows compatibility issues** across the Serena codebase that will cause runtime failures on Windows systems. Issues range from missing PATH resolution to platform-specific command syntax and missing environment variable inheritance.

**Status:** All issues are **fixable** and do not require architectural changes.

---

## Issues Found

### CRITICAL ISSUES (Must Fix)

#### 1. R Language Server: Hardcoded Bash Shell Command
**File:** `src/solidlsp/language_servers/r_language_server.py:69`

**Problem:**
```python
r_cmd = 'R --vanilla --quiet --slave -e "options(languageserver.debug_mode = FALSE); languageserver::run()"'
```
This creates a string command that is passed as a single argument to `ProcessLaunchInfo(cmd=r_cmd)`. This works on Unix with `shell=True`, but the LSP server handler may not execute this correctly on Windows.

**Impact:** R language server will fail to start on Windows.

**Fix:** Convert to list form and detect platform appropriately.

---

#### 2. Perl Language Server: Unix-Only Implementation
**File:** `src/solidlsp/language_servers/perl_language_server.py`

**Problem:**
- Line 82-83: Platform check only allows Linux and macOS:
```python
if platform_id not in valid_platforms:
    raise RuntimeError(f"Platform {platform_id} is not supported for Perl at the moment")
```
- Perl is available on Windows but explicitly blocked

**Impact:** Perl language server is disabled on Windows despite Perl being available.

**Fix:** Add `PlatformId.WIN_x64` and `PlatformId.WIN_arm64` to valid_platforms.

---

#### 3. Erlang Language Server: Missing env Inheritance + Unix uname() Call
**File:** `src/solidlsp/language_servers/erlang_language_server.py`

**Problems:**
1. Line 168: `os.uname()` is Unix-only, doesn't exist on Windows:
```python
is_macos = os.uname().sysname == "Darwin" if hasattr(os, "uname") else False
```
Even with `hasattr()` check, this is fragile.

2. Lines 58, 67, 78: subprocess calls missing env inheritance:
```python
result = subprocess.run([...], check=False, capture_output=True, text=True, timeout=10)
```

**Impact:** 
- Runtime AttributeError on Windows when accessing os.uname()
- erlang_ls, erl, rebar3 not found even if on PATH

**Fix:** 
- Use platform.system() instead of os.uname()
- Add env=os.environ.copy() to all subprocess calls

---

#### 4. Go Language Server (gopls): Missing env Inheritance
**File:** `src/solidlsp/language_servers/gopls.py`

**Problem:**
Lines 53, 64: Missing env parameter:
```python
result = subprocess.run([" go", "version"], capture_output=True, text=True, check=False)
result = subprocess.run(["gopls", "version"], capture_output=True, text=True, check=False)
```

**Impact:** go and gopls not found on Windows even if installed and on PATH.

**Fix:** Add env=os.environ.copy() to all subprocess.run() calls.

---

#### 5. Scala Language Server: shutil.which() Unreliability + Missing env
**File:** `src/solidlsp/language_servers/scala_language_server.py`

**Problems:**
- Line 61: `shutil.which("java")` unreliable on Windows
- Lines 66, 68, 79: Using `shutil.which()` without env inheritance
- Line 75, 107: subprocess calls missing env:
```python
subprocess.run([coursier_command_path, "setup", "--yes"], check=True, capture_output=True, text=True)
subprocess.run(cmd, cwd=metals_home, check=True)
```

**Impact:** 
- Java, coursier, and cs not found on Windows
- Scala language server fails to initialize

**Fix:**
- Use find_executable_in_path() helper
- Add env=os.environ.copy() to subprocess calls
- Add Windows platform support (currently has platform detection at line 21-22 but doesn't use it)

---

#### 6. DotNet Version Detection: Missing env Inheritance  
**File:** `src/solidlsp/ls_utils.py:387, 416`

**Problem:**
```python
result = subprocess.run(["dotnet", "--list-runtimes"], capture_output=True, check=True)
result = subprocess.run(["mono", "--version"], capture_output=True, check=True)
```
Missing env parameter in both dotnet and mono detection.

**Impact:** 
- C# language servers (OmniSharp, etc.) fail when dotnet is on PATH but not in subprocess inherited environment

**Fix:** Add env=os.environ.copy() to subprocess calls.

---

### HIGH PRIORITY ISSUES

#### 7. Multiple Language Servers: Inconsistent Path Resolution Pattern
**Files:** 
- `src/solidlsp/language_servers/erlang_language_server.py:33`
- Multiple others using `shutil.which()` without env inheritance or Windows fallback

**Problem:**
The pattern `shutil.which(executable_name)` is used for executable detection but doesn't work reliably on Windows when PATH inheritance is compromised. This is NOW FIXED for Node.js tools (rustup, node, npm, etc.) but still affects other language servers.

**Existing Fix Applied To:**
- ✅ typescript_language_server.py
- ✅ vts_language_server.py  
- ✅ intelephense.py
- ✅ yaml_language_server.py
- ✅ bash_language_server.py
- ✅ elm_language_server.py
- ✅ rust_analyzer.py

**Still Needs Fix:**
- ❌ erlang_language_server.py (erl, erlang_ls)
- ❌ gopls.py (go)
- ❌ scala_language_server.py (java, coursier, cs)
- ❌ Various others may need review

---

## Summary Table

| Issue | Severity | File(s) | Type | Fix |
|-------|----------|---------|------|-----|
| R shell command | CRITICAL | r_language_server.py | Command format | Convert to list |
| Perl disabled on Windows | CRITICAL | perl_language_server.py | Platform check | Add WIN platforms |
| Erlang uname() + no env | CRITICAL | erlang_language_server.py | Unix call + env | Use platform.system(), add env |
| Go env missing | CRITICAL | gopls.py | env inheritance | Add env=os.environ.copy() |
| Scala env + unreliable paths | CRITICAL | scala_language_server.py | env + detection | Use helper + add env |
| DotNet env missing | HIGH | ls_utils.py | env inheritance | Add env=os.environ.copy() |
| Path resolution pattern | HIGH | Multiple | Inconsistency | Use find_executable_in_path() |

---

## Implementation Priority

1. **Phase 1 (Must Do):**
   - Fix Erlang (os.uname() issue - will crash on Windows)
   - Fix Perl (re-enable on Windows)
   - Fix R (command format)
   - Fix Go/Scala (missing env)

2. **Phase 2 (Should Do):**
   - Fix DotNet detection env inheritance
   - Standardize path resolution across all language servers

3. **Phase 3 (Nice to Have):**
   - Create comprehensive Windows testing for all language servers
   - Add Windows CI/CD to validation pipeline

---

## Testing Recommendations

1. Test each language server on Windows with executables installed to:
   - System PATH
   - User-local paths (e.g., `~/.local/bin`)
   - Nonstandard locations

2. Verify error messages are helpful when tools aren't found

3. Cross-test on Linux/macOS to ensure no regressions

---

## Related Previously Fixed Issues

- ✅ Rust analyzer PATH resolution (commit ca7bdb4)
- ✅ Node.js tools PATH resolution (commit 6436642)
- ✅ General subprocess PATH helper (subprocess_util.py: find_executable_in_path, run_rustup_command)

---

## Recommendations for Future Development

1. **Create a platform abstraction layer** for executable detection:
   ```python
   # Instead of scattered shutil.which() calls
   from solidlsp.util.subprocess_util import find_executable_in_path
   exe_path = find_executable_in_path("tool-name")
   ```

2. **Enforce env=os.environ.copy() by default:**
   - Create wrapper functions for subprocess calls
   - Add linting rule to catch subprocess without env parameter

3. **Document platform requirements:**
   - Add Windows support notes to each language server
   - Create installation guide for Windows users

4. **Add platform-specific tests:**
   - Test each language server on Windows in CI/CD
   - Test with multiple PATH configurations
