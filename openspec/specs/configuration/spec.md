# configuration Specification

## Purpose
TBD - created by archiving change add-unfk-cli. Update Purpose after archive.
## Requirements
### Requirement: Configuration File Support
The tool SHALL support optional configuration files for project-specific settings.

#### Scenario: Config file discovery
- **WHEN** the tool runs in a directory
- **THEN** it looks for `.unfkrc`, `.unfkrc.toml`, or `unfk.toml` in the current directory and parent directories

#### Scenario: Config file format
- **WHEN** a config file is found
- **THEN** it is parsed as TOML format

#### Scenario: No config file required
- **WHEN** no config file is found
- **THEN** the tool operates with built-in defaults
- **AND** no warning is displayed

#### Scenario: Explicit config path
- **WHEN** user specifies `--config=path/to/config.toml`
- **THEN** the specified config file is used
- **AND** automatic config discovery is skipped

### Requirement: Configuration Hierarchy
The tool SHALL follow a configuration precedence hierarchy.

#### Scenario: CLI overrides config
- **WHEN** a setting is specified both in config file and CLI flag
- **THEN** the CLI flag value takes precedence

#### Scenario: Config overrides EditorConfig
- **WHEN** a setting is specified in unfk config file but not CLI
- **THEN** the unfk config file value takes precedence over EditorConfig

#### Scenario: EditorConfig overrides defaults
- **WHEN** a setting is specified in EditorConfig but not in CLI or unfk config
- **THEN** the EditorConfig value takes precedence over built-in defaults

#### Scenario: Full precedence chain
- **WHEN** determining the effective value for a setting
- **THEN** the precedence order is: CLI flags > unfk config file > EditorConfig > built-in defaults

### Requirement: Global Settings
The tool SHALL support global configuration settings.

#### Scenario: Global line ending setting
- **WHEN** config contains `line-ending = "lf"`
- **THEN** LF is applied to all files unless overridden

#### Scenario: Global indent setting
- **WHEN** config contains `indent = { style = "spaces", width = 2 }`
- **THEN** 2-space indentation is applied to all files unless overridden

#### Scenario: Global encoding setting
- **WHEN** config contains `encoding = "utf-8"`
- **THEN** UTF-8 encoding is enforced for all files

### Requirement: Per-Pattern Settings
The tool SHALL support glob pattern-based configuration overrides.

#### Scenario: Pattern-specific settings
- **WHEN** config contains a `[[rules]]` section with `pattern = "*.md"`
- **THEN** those settings apply only to matching files

#### Scenario: Multiple pattern rules
- **WHEN** config contains multiple `[[rules]]` sections
- **THEN** later rules override earlier rules for matching files

#### Scenario: Pattern negation
- **WHEN** config contains `pattern = "!*.min.js"`
- **THEN** matching files are excluded from processing

### Requirement: Ignore Configuration
The tool SHALL support configuring files to ignore.

#### Scenario: Ignore patterns in config
- **WHEN** config contains `ignore = ["node_modules", "*.lock"]`
- **THEN** matching files and directories are skipped

#### Scenario: Ignore file
- **WHEN** a `.unfkignore` file exists
- **THEN** patterns in it are added to the ignore list

### Requirement: Configuration Validation
The tool SHALL validate configuration files.

#### Scenario: Invalid config error
- **WHEN** a config file contains invalid TOML syntax
- **THEN** a clear error message is displayed
- **AND** the tool exits with code 2

#### Scenario: Unknown setting warning
- **WHEN** a config file contains an unrecognized setting
- **THEN** a warning is displayed
- **AND** processing continues

### Requirement: Configuration Generation
The tool SHALL help users create configuration files.

#### Scenario: Init command
- **WHEN** user runs `unfk init`
- **THEN** a default `.unfkrc.toml` file is created in the current directory

#### Scenario: Init with analysis
- **WHEN** user runs `unfk init --analyze`
- **THEN** the tool scans existing files
- **AND** generates a config file reflecting current conventions

#### Scenario: Config dump
- **WHEN** user runs `unfk config --dump`
- **THEN** the effective configuration (merged from all sources) is displayed

### Requirement: Example Configuration
The tool SHALL support a well-documented configuration format.

#### Scenario: Example config structure
- **WHEN** user requests example configuration
- **THEN** an example like the following is provided:

```toml
# Global settings
line-ending = "lf"
encoding = "utf-8"
final-newline = true
trailing-whitespace = "remove"

[indent]
style = "spaces"
width = 2

# Ignore patterns
ignore = [
  "node_modules",
  "target",
  "*.min.js",
]

# Pattern-specific overrides
[[rules]]
pattern = "Makefile"
indent = { style = "tabs" }

[[rules]]
pattern = "*.py"
indent = { style = "spaces", width = 4 }

[[rules]]
pattern = "*.bat"
line-ending = "crlf"
```

### Requirement: EditorConfig File Discovery
The tool SHALL discover and parse `.editorconfig` files following the EditorConfig specification.

#### Scenario: EditorConfig file found
- **WHEN** an `.editorconfig` file exists in the file's directory or any parent directory
- **THEN** the file is parsed and its settings are applied to matching files

#### Scenario: Multiple EditorConfig files
- **WHEN** multiple `.editorconfig` files exist in the directory hierarchy
- **THEN** settings are merged with closer files taking precedence over farther ones

#### Scenario: Root EditorConfig
- **WHEN** an `.editorconfig` file contains `root = true`
- **THEN** the search for parent `.editorconfig` files stops at that file

#### Scenario: No EditorConfig file
- **WHEN** no `.editorconfig` file exists in the directory hierarchy
- **THEN** EditorConfig contributes no settings to the configuration

### Requirement: EditorConfig Property Mapping
The tool SHALL map EditorConfig properties to unfk settings.

#### Scenario: Line ending mapping
- **WHEN** EditorConfig specifies `end_of_line = lf`
- **THEN** the line ending setting is set to LF
- **AND** `end_of_line = crlf` maps to CRLF

#### Scenario: Indent style mapping
- **WHEN** EditorConfig specifies `indent_style = tab`
- **THEN** the indent style is set to Tabs
- **AND** `indent_style = space` maps to Spaces

#### Scenario: Indent size mapping
- **WHEN** EditorConfig specifies `indent_size = 4`
- **THEN** the indent width is set to 4

#### Scenario: Charset mapping
- **WHEN** EditorConfig specifies `charset = utf-8`
- **THEN** the encoding setting is set to utf-8

#### Scenario: Trailing whitespace mapping
- **WHEN** EditorConfig specifies `trim_trailing_whitespace = true`
- **THEN** trailing whitespace handling is set to Remove
- **AND** `trim_trailing_whitespace = false` maps to Keep

#### Scenario: Final newline mapping
- **WHEN** EditorConfig specifies `insert_final_newline = true`
- **THEN** the final newline setting is set to true
- **AND** `insert_final_newline = false` maps to false

### Requirement: EditorConfig Disable Flag
The tool SHALL allow disabling EditorConfig integration via CLI.

#### Scenario: Disable EditorConfig
- **WHEN** user specifies `--no-editorconfig` flag
- **THEN** EditorConfig files are not read or applied
- **AND** only unfk config and CLI flags determine settings

