# Capability: Line Ending Repair

## ADDED Requirements

### Requirement: Line Ending Detection
The tool SHALL detect line ending styles used in files.

#### Scenario: LF detection
- **WHEN** a file uses LF (`\n`) line endings exclusively
- **THEN** the file is detected as using LF style

#### Scenario: CRLF detection
- **WHEN** a file uses CRLF (`\r\n`) line endings exclusively
- **THEN** the file is detected as using CRLF style

#### Scenario: Mixed line ending detection
- **WHEN** a file contains both LF and CRLF line endings
- **THEN** the file is flagged as having mixed/inconsistent line endings

#### Scenario: CR-only detection
- **WHEN** a file uses CR (`\r`) only line endings (legacy Mac)
- **THEN** the file is flagged as using deprecated CR style

### Requirement: Line Ending Normalization
The tool SHALL normalize line endings to a consistent style.

#### Scenario: Convert mixed to LF
- **WHEN** fixing a file with mixed line endings
- **AND** the target style is LF (default)
- **THEN** all line endings are converted to LF

#### Scenario: Convert to CRLF
- **WHEN** fixing a file with `--line-ending=crlf`
- **THEN** all line endings are converted to CRLF

#### Scenario: Auto-detect target style
- **WHEN** fixing a file with `--line-ending=auto`
- **THEN** the dominant line ending style in the file is used as the target
- **AND** if no dominant style exists, the file-type default is used

### Requirement: File-Type Line Ending Defaults
The tool SHALL apply appropriate default line endings based on file type.

#### Scenario: Default LF for most files
- **WHEN** processing most file types (source code, config, markdown, etc.)
- **THEN** LF is the default target line ending

#### Scenario: Default CRLF for Windows batch files
- **WHEN** processing `.bat` or `.cmd` files
- **THEN** CRLF is the default target line ending

#### Scenario: Default CRLF for PowerShell scripts
- **WHEN** processing `.ps1`, `.psm1`, or `.psd1` files
- **THEN** CRLF is the default target line ending

### Requirement: Line Ending Report
The tool SHALL report line ending issues clearly.

#### Scenario: Report mixed line endings
- **WHEN** scanning a file with mixed line endings
- **THEN** the report shows: "Mixed line endings (LF and CRLF)"

#### Scenario: Report unexpected line ending style
- **WHEN** scanning a file with line endings that differ from the expected style
- **THEN** the report shows: "Expected LF, found CRLF" (or vice versa)

#### Scenario: Show line counts
- **WHEN** running in verbose mode
- **THEN** the report includes counts: "Found 50 LF, 3 CRLF"
