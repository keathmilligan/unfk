<picture>
  <source srcset="unfk-lt.png" media="(prefers-color-scheme: dark)">
  <source srcset="unfk-dk.png" media="(prefers-color-scheme: light)">
  <img src="unfk-lt.png" alt="OpenCode logo">
</picture>

_`UNiversal File... Korrector?`_

A fast, modern CLI tool for scanning and repairing file formatting issues

---

## Overview

`unfk` detects and fixes common file formatting inconsistencies across your codebase:

- **Line endings** — mixed or incorrect line endings (LF vs CRLF)
- **Indentation** — inconsistent tabs/spaces or wrong indent width
- **Encoding** — non-UTF-8 files or encoding mismatches
- **Final newlines** — missing or extra newlines at end of file
- **Trailing whitespace** — spaces/tabs at end of lines
- **Blank lines (Markdown)** — multiple successive blank lines in `.md` files

Unlike code formatters such as Prettier or Black, `unfk` is not concerned with code style or syntax. Instead, it focuses on low-level file hygiene issues that affect a much broader range of file types—config files, scripts, data files, documentation, and more.

`unfk` is aware of file type-specific conventions and applies the right defaults automatically. For example:

- Windows batch files (`.bat`, `.cmd`) and PowerShell scripts require CRLF line endings
- `Makefile` requires tabs for indentation
- `.go` files use tabs by convention, while `.py` uses 4 spaces and `.rb` uses 2
- `.vb` and `.reg` files expect CRLF

This means you can run `unfk fix` across a mixed codebase and trust it to do the right thing for each file type.

### Markdown-Specific Features

For markdown files (`.md`), `unfk` includes additional checks:

- **Successive blank lines**: Detects and optionally fixes multiple consecutive blank lines (double-spacing or more)
- Single blank lines between paragraphs are preserved (standard markdown practice)
- Two trailing spaces (markdown line breaks) are preserved by default

Example:
```bash
# Scan for issues including successive blank lines
unfk scan document.md

# Fix all issues including successive blank lines
unfk fix --all document.md
```

## Installation

### cargo

```bash
cargo install unfk
```

### Shell installer (macOS / Linux)

```bash
curl -fsSL https://install.keathmilligan.dev/unfk/install.sh | sh
```

### PowerShell installer (Windows)

```powershell
irm https://install.keathmilligan.dev/unfk/install.ps1 | iex
```

### Windows MSI

Download the signed `.msi` installer directly from the [GitHub Releases](https://github.com/keathmilligan/unfk/releases) page.

### Homebrew (macOS / Linux)

```bash
brew tap keathmilligan/tap
brew install keathmilligan/tap/unfk
```

### Scoop (Windows)

```powershell
scoop bucket add keathmilligan https://github.com/keathmilligan/scoop-bucket
scoop install unfk
```

### Chocolatey (Windows)

```powershell
choco install unfk
```

### winget (Windows)

```powershell
winget install keathmilligan.unfk
```

### apt (Debian / Ubuntu)

```bash
curl -fsSL https://install.keathmilligan.dev/gpg.key | sudo gpg --dearmor -o /etc/apt/keyrings/keathmilligan.gpg
echo "deb [signed-by=/etc/apt/keyrings/keathmilligan.gpg] https://install.keathmilligan.dev/apt stable main" | sudo tee /etc/apt/sources.list.d/keathmilligan.list
sudo apt update
sudo apt install unfk
```

### dnf / rpm (Fedora / RHEL / CentOS)

```bash
sudo curl -o /etc/yum.repos.d/keathmilligan.repo https://install.keathmilligan.dev/rpm/keathmilligan.repo
sudo dnf install unfk
```

### AUR (Arch Linux)

```bash
yay -S unfk-bin
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

### Why switch?

- **File-type awareness** — `unfk` automatically uses the correct line ending for each file type. Windows batch files stay CRLF even when you normalize everything else to LF.
- **Batch processing** — Fix entire directories recursively with proper gitignore support.
- **More than line endings** — While you're at it, fix indentation, encoding, final newlines, and trailing whitespace too.
- **Dry-run mode** — Preview changes with `--dry-run` before modifying files.
- **Modern defaults** — Sensible out-of-the-box behavior for contemporary development workflows.

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
