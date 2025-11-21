<picture>
  <source srcset="unfk-lt.png" media="(prefers-color-scheme: dark)">
  <source srcset="unfk-dk.png" media="(prefers-color-scheme: light)">
  <img src="unfk-lt.png" alt="OpenCode logo">
</picture>

A fast, modern CLI tool for scanning and repairing file formatting issues

---

## Overview

**unfk** detects and fixes common file formatting inconsistencies across your codebase:

- **Line endings** — mixed or incorrect line endings (LF vs CRLF)
- **Indentation** — inconsistent tabs/spaces or wrong indent width
- **Encoding** — non-UTF-8 files or encoding mismatches
- **Final newlines** — missing or extra newlines at end of file
- **Trailing whitespace** — spaces/tabs at end of lines

Unlike code formatters such as Prettier or Black, unfk is not concerned with code style or syntax. Instead, it focuses on low-level file hygiene issues that affect a much broader range of file types—config files, scripts, data files, documentation, and more.

unfk is aware of file type-specific conventions and applies the right defaults automatically. For example:

- Windows batch files (`.bat`, `.cmd`) and PowerShell scripts require CRLF line endings
- `Makefile` requires tabs for indentation
- `.go` files use tabs by convention, while `.py` uses 4 spaces and `.rb` uses 2
- `.vb` and `.reg` files expect CRLF

This means you can run `unfk fix` across a mixed codebase and trust it to do the right thing for each file type.

## Installation

```bash
cargo install unfk
```

## Quick Start

```bash
# Scan current directory for issues
unfk

# Fix all issues
unfk fix

# Preview changes without modifying files
unfk fix --dry-run

# Create a configuration file
unfk init
```

## Commands

| Command | Description |
|---------|-------------|
| `unfk` / `unfk scan` | Scan for formatting issues |
| `unfk fix` | Automatically repair issues |
| `unfk init` | Create `.unfkrc.toml` config file |
| `unfk types` | List supported file types |
| `unfk config` | Show current configuration |

## Configuration

A config file is not required — unfk assumes sane defaults for most file types and modern development conventions. For custom rules, create `.unfkrc.toml` in your project root:

```toml
line-ending = "lf"
encoding = "utf-8"
final-newline = true
trailing-whitespace = "remove"

[indent]
style = "spaces"
width = 2

# Per-pattern overrides
[[rules]]
pattern = "*.py"
[rules.indent]
style = "spaces"
width = 4
```

## License

MIT

