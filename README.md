<picture>
  <source srcset="unfk-lt.png" media="(prefers-color-scheme: dark)">
  <source srcset="unfk-dk.png" media="(prefers-color-scheme: light)">
  <img src="unfk-lt.png" alt="OpenCode logo">
</picture>

[![CI](https://github.com/keathmilligan/unfk/actions/workflows/ci.yml/badge.svg)](https://github.com/keathmilligan/unfk/actions/workflows/ci.yml)
[![Release](https://github.com/keathmilligan/unfk/actions/workflows/release.yml/badge.svg)](https://github.com/keathmilligan/unfk/actions/workflows/release.yml)
[![macOS DMG](https://packages.keathmilligan.net/unfk/badges/macos-dmg.svg)](https://github.com/keathmilligan/unfk/releases/latest)
[![Windows MSI](https://packages.keathmilligan.net/unfk/badges/msi.svg)](https://github.com/keathmilligan/unfk/releases/latest)
[![Homebrew](https://packages.keathmilligan.net/unfk/badges/homebrew.svg)](https://github.com/keathmilligan/unfk/releases/latest)
[![Scoop](https://packages.keathmilligan.net/unfk/badges/scoop.svg)](https://github.com/keathmilligan/unfk/releases/latest)
[![apt](https://packages.keathmilligan.net/unfk/badges/apt.svg)](https://github.com/keathmilligan/unfk/releases/latest)
[![rpm](https://packages.keathmilligan.net/unfk/badges/rpm.svg)](https://github.com/keathmilligan/unfk/releases/latest)
[![crates.io](https://packages.keathmilligan.net/unfk/badges/crates-io.svg)](https://github.com/keathmilligan/unfk/releases/latest)
[![Install Scripts](https://packages.keathmilligan.net/unfk/badges/install-scripts.svg)](https://github.com/keathmilligan/unfk/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

_`UNiversal File... Korrector?`_

A fast CLI tool for scanning and repairing file formatting issues in a broad variety of file types.

---

## Overview

`unfk` detects and fixes common file formatting inconsistencies:

- **Line endings** — mixed or incorrect line endings (LF vs CRLF)
- **Indentation** — inconsistent tabs/spaces or wrong indent width
- **Encoding** — non-UTF-8 files or encoding mismatches
- **Final newlines** — missing or extra newlines at end of file
- **Trailing whitespace** — spaces/tabs at end of lines
- **Blank lines (Markdown)** — multiple successive blank lines in `.md` files

Unlike code formatters such as Prettier or Black, `unfk` is not concerned with code style or syntax. Instead, it focuses on general formatting and file hygiene issues that affect a much broader range of file types — config files, scripts, data files, documentation, and more.

`unfk` is aware of file type-specific conventions and applies the right defaults automatically. For example:

- `.go` files use tabs by convention, while `.py` and `.rs` use 4 spaces and `.rb` uses 2, etc.
- `Makefile` requires tabs for indentation
- `.vb` and `.reg` files require CRLF line-endings
- Windows batch files (`.bat`, `.cmd`) and PowerShell scripts (`.ps1`) require CRLF line endings

This means you can run `unfk fix` across a mixed codebase and trust it to do the right thing for each file type.

Example:
```bash
# Scan and optionally fix files recursively
unfk

# Scan a file or directory
unfk scan document.md

# Fix all issues
unfk fix --all document.md
```

`unfk` is written in Rust and runs on macOS, Linux and Windows (Intel, ARM and Apple Silicon).

## Installation and Update

There are many ways to install `unfk` depending on your platform.

### macOS (Homebrew)

```bash
brew tap keathmilligan/tap
brew install keathmilligan/tap/unfk
```

Stay up-to-date with `brew upgrade unfk`.

See the [macOS Install Guide](docs/install-macos.md) for other ways to install on macOS.

### Windows (PowerShell)

In an elevated powershell session, run:

```powershell
irm https://packages.keathmilligan.net/unfk/install.ps1 | iex
```

See the [Windows Install Guide](docs/install-windows.md) for other ways to install on Windows.

### Linux (shell installer)

```bash
curl -fsSL https://packages.keathmilligan.net/unfk/install.sh | sh
```

This will install `unfk` into `~/.local/bin`.

See the [Linux Install Guide](docs/install-linux.md) for other ways to install on Linux.

## Quick Start

```bash
# Scan current directory and optionally fix issues
unfk

# Scan a file or directory
unfk scan docs

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
| `unfk` | Scan for issues and optionally fix|
| `unfk scan` | Scan for issues |
| `unfk fix` | Automatically repair issues |
| `unfk init` | Create `.unfkrc.toml` config file |
| `unfk types` | List supported file types |
| `unfk config` | Show current configuration |

## Configuration

A config file is not required — `unfk` assumes sane defaults for most file types and modern development conventions.

### EditorConfig Support

`unfk` automatically reads `.editorconfig` files if present. This lets you share formatting settings across tools and editors without duplicating configuration. The following EditorConfig properties are supported:

| EditorConfig Property       | `unfk` Setting            |
|-----------------------------|---------------------------|
| `end_of_line`               | Line ending style         |
| `indent_style`              | Tabs or spaces            |
| `indent_size`               | Indent width              |
| `charset`                   | File encoding             |
| `trim_trailing_whitespace`  | Trailing whitespace       |
| `insert_final_newline`      | Final newline             |

**Precedence order:** CLI flags > `.unfkrc.toml` > `.editorconfig` > built-in defaults

To disable EditorConfig integration, use `--no-editorconfig`.

### Custom Configuration

For custom rules, create `.unfkrc.toml` in your project root:

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

## Replacing Legacy Tools

`unfk` can replace several single-purpose legacy utilities with one modern, file-type-aware tool:

| Legacy Tool | `unfk` Equivalent |
|-------------|-------------------|
| `dos2unix` | `unfk fix --line-ending lf` |
| `unix2dos` | `unfk fix --line-ending crlf` |
| `mac2unix` | `unfk fix --line-ending lf` |
| `fromdos` / `todos` | `unfk fix --line-ending lf` / `crlf` |

```bash
# Convert all files to LF (respecting file-type conventions)
unfk fix --line-ending lf

# Convert specific files to CRLF
unfk fix --line-ending crlf src/scripts/*.bat

# Preview what would change
unfk fix --line-ending lf --dry-run
```

## License

MIT
