# Change: Add unfk CLI Tool

## Why

Text files across projects often suffer from inconsistent formatting issues—mixed line endings, inconsistent indentation, incorrect encoding, and trailing whitespace—that cause merge conflicts, build failures, and cross-platform compatibility problems. Existing tools like Prettier focus on code formatting for specific languages, leaving a gap for general file-hygiene fixes across diverse file types. `unfk` fills this gap with a fast, zero-configuration tool that scans and repairs these fundamental issues.

## What Changes

- **NEW**: Complete Rust-based CLI tool for file formatting repair
- **NEW**: Line-ending detection and repair (LF, CRLF, mixed)
- **NEW**: Indentation style detection and repair (tabs vs spaces)
- **NEW**: Encoding detection and repair (UTF-8, UTF-16, Latin-1, etc.)
- **NEW**: End-of-file cleanup (trailing blank lines, missing final newline)
- **NEW**: Intelligent file-type defaults (Makefiles use tabs, most files use LF, etc.)
- **NEW**: Scan mode (report issues without modifying) and fix mode (auto-repair)
- **NEW**: Command-line switches to override defaults
- **NEW**: Optional configuration file for project-specific overrides

## Impact

- Affected specs: Creates 8 new capability specs
  - `cli-interface` - Command-line interface design
  - `file-scanning` - File detection and traversal
  - `line-ending-repair` - Line ending normalization
  - `indentation-repair` - Indentation style enforcement
  - `encoding-repair` - Character encoding normalization
  - `eof-repair` - End-of-file formatting
  - `file-type-defaults` - Per-filetype default configurations
  - `configuration` - Configuration file support
- Affected code: New Rust project (Cargo workspace)
- Dependencies: Rust ecosystem crates for CLI, file handling, encoding detection
