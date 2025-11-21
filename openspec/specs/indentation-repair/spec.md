# indentation-repair Specification

## Purpose
TBD - created by archiving change add-unfk-cli. Update Purpose after archive.
## Requirements
### Requirement: Indentation Detection
The tool SHALL detect indentation styles used in files.

#### Scenario: Tab indentation detection
- **WHEN** a file uses tabs for indentation exclusively
- **THEN** the file is detected as using tab indentation

#### Scenario: Space indentation detection
- **WHEN** a file uses spaces for indentation exclusively
- **THEN** the file is detected as using space indentation
- **AND** the space width (2, 4, 8, etc.) is detected

#### Scenario: Mixed indentation detection
- **WHEN** a file contains both tabs and spaces for indentation
- **THEN** the file is flagged as having mixed/inconsistent indentation

#### Scenario: Inline tab detection
- **WHEN** tabs appear in the middle of lines (not at start)
- **THEN** these are not considered indentation issues by default

### Requirement: Indentation Normalization
The tool SHALL normalize indentation to a consistent style.

#### Scenario: Convert tabs to spaces
- **WHEN** fixing a file with `--indent=spaces:4`
- **THEN** all leading tabs are converted to 4 spaces each

#### Scenario: Convert spaces to tabs
- **WHEN** fixing a file with `--indent=tabs`
- **AND** the file uses space indentation
- **THEN** leading spaces are converted to tabs (using detected or specified width)

#### Scenario: Preserve alignment spaces
- **WHEN** converting indentation
- **THEN** alignment spaces within code (e.g., aligning = signs) are preserved where possible

### Requirement: File-Type Indentation Defaults
The tool SHALL apply appropriate default indentation based on file type.

#### Scenario: Tabs required for Makefiles
- **WHEN** processing `Makefile`, `makefile`, or `*.mk` files
- **THEN** tabs are required for indentation
- **AND** spaces in recipe lines are flagged as errors

#### Scenario: Tabs for Go files
- **WHEN** processing `*.go` files
- **THEN** tabs are the default indentation style

#### Scenario: Spaces for Python files
- **WHEN** processing `*.py` files
- **THEN** spaces (4-space width) are the default indentation style

#### Scenario: Spaces for YAML files
- **WHEN** processing `*.yml` or `*.yaml` files
- **THEN** spaces (2-space width) are the default indentation style

#### Scenario: Generic default
- **WHEN** processing files without a specific indentation convention
- **THEN** spaces (2-space width) are the default

### Requirement: Indentation Width Configuration
The tool SHALL support configuring indentation width.

#### Scenario: Specify space width
- **WHEN** user specifies `--indent=spaces:2`
- **THEN** 2-space indentation is enforced

#### Scenario: Tab width for display
- **WHEN** user specifies `--tab-width=4`
- **THEN** tabs are treated as 4-character width for alignment calculations

### Requirement: Indentation Report
The tool SHALL report indentation issues clearly.

#### Scenario: Report mixed indentation
- **WHEN** scanning a file with mixed indentation
- **THEN** the report shows: "Mixed indentation (tabs and spaces)"

#### Scenario: Report wrong indentation style
- **WHEN** scanning a file with indentation that differs from expected
- **THEN** the report shows: "Expected tabs, found spaces" (or vice versa)

#### Scenario: Report inconsistent width
- **WHEN** scanning a file with inconsistent space indentation widths
- **THEN** the report shows: "Inconsistent indentation width (found 2, 3, and 4 spaces)"

