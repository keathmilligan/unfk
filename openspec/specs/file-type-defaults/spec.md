# file-type-defaults Specification

## Purpose
TBD - created by archiving change add-unfk-cli. Update Purpose after archive.
## Requirements
### Requirement: Built-in File Type Registry
The tool SHALL maintain a registry of file types with their default settings.

#### Scenario: Source code files recognized
- **WHEN** processing files with extensions like `.rs`, `.py`, `.js`, `.ts`, `.go`, `.c`, `.cpp`, `.java`
- **THEN** the file type is recognized and appropriate defaults are applied

#### Scenario: Configuration files recognized
- **WHEN** processing files with extensions like `.json`, `.yaml`, `.yml`, `.toml`, `.xml`, `.ini`
- **THEN** the file type is recognized and appropriate defaults are applied

#### Scenario: Documentation files recognized
- **WHEN** processing files with extensions like `.md`, `.txt`, `.rst`, `.adoc`
- **THEN** the file type is recognized and appropriate defaults are applied

#### Scenario: Script files recognized
- **WHEN** processing files with extensions like `.sh`, `.bash`, `.zsh`, `.bat`, `.cmd`, `.ps1`
- **THEN** the file type is recognized and appropriate defaults are applied

### Requirement: Default Line Endings by Type
The tool SHALL apply appropriate line ending defaults per file type.

#### Scenario: LF default for Unix scripts
- **WHEN** processing `.sh`, `.bash`, `.zsh` files
- **THEN** LF line endings are the default

#### Scenario: CRLF default for Windows scripts
- **WHEN** processing `.bat`, `.cmd` files
- **THEN** CRLF line endings are the default

#### Scenario: CRLF default for PowerShell
- **WHEN** processing `.ps1`, `.psm1`, `.psd1` files
- **THEN** CRLF line endings are the default

#### Scenario: LF default for most source code
- **WHEN** processing `.rs`, `.py`, `.js`, `.ts`, `.go`, `.c`, `.java`, etc.
- **THEN** LF line endings are the default

### Requirement: Default Indentation by Type
The tool SHALL apply appropriate indentation defaults per file type.

#### Scenario: Tabs for Makefiles
- **WHEN** processing `Makefile`, `makefile`, `GNUmakefile`, `*.mk` files
- **THEN** tabs are required (not just default)

#### Scenario: Tabs for Go
- **WHEN** processing `*.go` files
- **THEN** tabs are the default indentation

#### Scenario: 4-space for Python
- **WHEN** processing `*.py` files
- **THEN** 4-space indentation is the default

#### Scenario: 2-space for YAML
- **WHEN** processing `*.yaml`, `*.yml` files
- **THEN** 2-space indentation is the default

#### Scenario: 2-space for JSON
- **WHEN** processing `*.json` files
- **THEN** 2-space indentation is the default

#### Scenario: 2-space for JavaScript/TypeScript
- **WHEN** processing `*.js`, `*.ts`, `*.jsx`, `*.tsx` files
- **THEN** 2-space indentation is the default

#### Scenario: 4-space for Rust
- **WHEN** processing `*.rs` files
- **THEN** 4-space indentation is the default

### Requirement: Special File Handling
The tool SHALL handle special cases for specific file types.

#### Scenario: Markdown trailing spaces
- **WHEN** processing markdown files (`.md`)
- **THEN** two trailing spaces (line break) are preserved by default

#### Scenario: EditorConfig awareness
- **WHEN** an `.editorconfig` file is present in the project
- **THEN** settings from `.editorconfig` are respected as defaults
- **AND** CLI flags still override `.editorconfig` settings

### Requirement: File Type Override
The tool SHALL allow overriding file type detection.

#### Scenario: Force file type
- **WHEN** user specifies `--type=python`
- **THEN** all processed files are treated as Python files regardless of extension

#### Scenario: Custom extension mapping
- **WHEN** user specifies `--ext-type=.custom:python`
- **THEN** `.custom` files are treated as Python files

### Requirement: File Type Listing
The tool SHALL allow users to see known file types and defaults.

#### Scenario: List known types
- **WHEN** user runs `unfk types`
- **THEN** a list of recognized file types is displayed

#### Scenario: Show type defaults
- **WHEN** user runs `unfk types --show=python`
- **THEN** the default settings for Python files are displayed

