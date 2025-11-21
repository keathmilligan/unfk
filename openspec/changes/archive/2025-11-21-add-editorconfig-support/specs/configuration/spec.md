## ADDED Requirements

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

## MODIFIED Requirements

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
