# Design: unfk CLI Tool

## Context

`unfk` is a fast, modern command-line tool for scanning and repairing general file-formatting issues. It targets text files across diverse types without requiring external tools or complex configuration.

### Stakeholders
- Developers working on cross-platform projects
- Teams with mixed editor/IDE preferences
- CI/CD pipelines needing file hygiene checks
- Open-source maintainers managing contributions

### Constraints
- Must be fast enough to run on large codebases (thousands of files)
- Must work without any configuration for common cases
- Must not break binary files or files it doesn't understand
- Must provide clear, actionable output

## Goals / Non-Goals

### Goals
- Fast execution via Rust implementation
- Zero-config operation with sensible defaults
- Comprehensive detection: line endings, indentation, encoding, EOF issues
- Both scan (report-only) and fix (auto-repair) modes
- Override defaults via CLI flags or config file
- Respect `.gitignore` and similar exclusion patterns
- Clear exit codes for CI integration

### Non-Goals
- Language-specific code formatting (use Prettier, Black, rustfmt, etc.)
- Syntax validation or linting
- Semantic code analysis
- Plugin/extension system (keep it simple)
- GUI interface

## Decisions

### Decision 1: Rust as Implementation Language
- **What**: Implement in Rust using Cargo
- **Why**: Performance (comparable to ripgrep), memory safety, excellent CLI ecosystem (clap), cross-platform binaries
- **Alternatives considered**:
  - Go: Good performance but less expressive for parsing
  - TypeScript/Node: Slower startup, larger distribution size
  - Python: Too slow for large codebases

### Decision 2: File Detection Strategy
- **What**: Use file extension mapping with fallback heuristics
- **Why**: Fast and predictable; covers 99% of cases
- **Details**:
  - Primary: Map extensions to file types (`.rs` → Rust, `.md` → Markdown, etc.)
  - Secondary: First-line detection (`#!/bin/bash`, `<?xml`, etc.)
  - Fallback: Treat as generic text if detected as text (not binary)
- **Alternatives considered**:
  - libmagic/file: Heavier dependency, slower
  - Content-only detection: Less predictable

### Decision 3: Binary File Detection
- **What**: Skip files detected as binary
- **Why**: Prevents corruption of images, compiled files, etc.
- **Details**:
  - Check for NULL bytes in first 8KB
  - Check for common binary signatures (PNG, JPEG, ELF, etc.)
  - Allow explicit include via CLI flag if needed

### Decision 4: Default Line Endings by Platform/Type
- **What**: Default to LF for most files, CRLF only for specific Windows-centric types
- **Why**: LF is the modern standard; Git normalizes to LF; most tools expect LF
- **Details**:
  - Default: LF for all files
  - Exceptions: `.bat`, `.cmd`, `.ps1` (Windows batch/PowerShell) default to CRLF
  - Override via `--line-ending=crlf|lf|auto`

### Decision 5: Default Indentation by File Type
- **What**: Apply per-filetype indentation defaults
- **Why**: Some formats require tabs (Makefile); others have strong conventions
- **Details**:
  - Makefile, Go: Tabs required
  - Python, YAML, JSON, most others: Spaces (configurable width, default 2 or 4)
  - Generic fallback: Detect from file content or use spaces
- **Override**: `--indent=tabs|spaces:N`

### Decision 6: Configuration Hierarchy
- **What**: CLI flags > config file > built-in defaults
- **Why**: Follows principle of least surprise; explicit overrides implicit
- **Details**:
  - Config file: `.unfkrc`, `.unfkrc.toml`, `unfk.toml` in project root
  - Format: TOML (simple, widely supported)
  - Supports per-glob overrides: `[*.md]` sections

### Decision 7: Output Modes
- **What**: Scan mode (default) reports issues; fix mode modifies files
- **Why**: Safe by default; users must opt-in to changes
- **Details**:
  - `unfk` or `unfk scan`: Report issues, exit non-zero if any found
  - `unfk fix`: Apply fixes, report what changed
  - `--dry-run`: Show what would change without modifying
  - `--quiet`: Suppress output (exit code only)
  - `--verbose`: Detailed per-file reporting

### Decision 8: Exit Codes
- **What**: Distinct exit codes for different outcomes
- **Why**: CI/CD integration; scripts can react appropriately
- **Details**:
  - `0`: No issues found (scan) or all fixes applied (fix)
  - `1`: Issues found (scan) or some files could not be fixed
  - `2`: Configuration/argument error
  - `3`: I/O or permission error

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Layer                           │
│  (clap: argument parsing, subcommands, help generation)     │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────┐
│                    Configuration Layer                       │
│  (config file loading, CLI override merging, defaults)       │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────┐
│                   File Discovery Layer                       │
│  (directory walking, gitignore respect, binary detection)    │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────┐
│                    Analysis Engine                           │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────────────┐ │
│  │ Line Endings │ │ Indentation  │ │ Encoding Detection   │ │
│  └──────────────┘ └──────────────┘ └──────────────────────┘ │
│  ┌──────────────┐ ┌──────────────┐                          │
│  │ EOF Issues   │ │ Trailing WS  │                          │
│  └──────────────┘ └──────────────┘                          │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────┐
│                     Repair Engine                            │
│  (applies fixes, writes files, generates diff output)        │
└─────────────────────────────────────────────────────────────┘
```

## Risks / Trade-offs

### Risk: Breaking Intentional Formatting
- **Issue**: Some files may intentionally use mixed line endings or specific encoding
- **Mitigation**: Provide granular ignore patterns; support inline `unfk:ignore` comments; default to scan-only mode

### Risk: Large File Performance
- **Issue**: Very large files (100MB+) could cause memory issues
- **Mitigation**: Stream processing where possible; skip files above configurable size limit (default 10MB)

### Risk: Encoding Detection Accuracy
- **Issue**: Encoding detection is heuristic and can be wrong
- **Mitigation**: Use well-tested library (encoding_rs); require high confidence; allow explicit override

### Trade-off: Simplicity vs. Flexibility
- **Decision**: Favor simplicity; keep config surface minimal
- **Consequence**: Power users may want features we won't add (plugins, custom rules)

## Migration Plan

Not applicable—this is a new tool with no existing users or data to migrate.

## Open Questions

1. **Backup strategy**: Should `unfk fix` create `.bak` files by default, or rely on version control?
   - Proposed: No backups by default; assume VCS; add `--backup` flag if requested

2. **Parallel processing**: How many worker threads for large directories?
   - Proposed: Default to CPU count; add `--jobs N` flag

3. **Editor integration**: Should we provide LSP or editor plugins?
   - Proposed: Defer to future; focus on CLI first

4. **Pre-commit hook**: Should we bundle a pre-commit hook definition?
   - Proposed: Yes, include `.pre-commit-hooks.yaml` for easy integration
