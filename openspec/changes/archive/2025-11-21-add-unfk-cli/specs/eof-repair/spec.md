# Capability: EOF Repair

## ADDED Requirements

### Requirement: Final Newline Detection
The tool SHALL detect missing or extra final newlines.

#### Scenario: Missing final newline detection
- **WHEN** a file does not end with a newline character
- **THEN** the file is flagged as missing a final newline

#### Scenario: Proper final newline
- **WHEN** a file ends with exactly one newline character
- **THEN** no issue is reported

#### Scenario: Multiple trailing newlines detection
- **WHEN** a file ends with multiple consecutive newline characters
- **THEN** the file is flagged as having extra trailing newlines

### Requirement: Final Newline Normalization
The tool SHALL ensure files end with exactly one newline.

#### Scenario: Add missing newline
- **WHEN** fixing a file missing a final newline
- **THEN** a single newline character is appended

#### Scenario: Remove extra newlines
- **WHEN** fixing a file with multiple trailing newlines
- **THEN** extra newlines are removed, leaving exactly one

#### Scenario: Respect line ending style
- **WHEN** adding a final newline
- **THEN** the newline character matches the file's line ending style (LF or CRLF)

### Requirement: Trailing Blank Lines Detection
The tool SHALL detect trailing blank lines at end of file.

#### Scenario: Single trailing blank line allowed
- **WHEN** a file ends with one blank line (content + newline + newline)
- **THEN** this is considered acceptable by default

#### Scenario: Multiple trailing blank lines
- **WHEN** a file has 2 or more trailing blank lines
- **THEN** the file is flagged as having excessive trailing blank lines

#### Scenario: Configurable blank line limit
- **WHEN** user specifies `--max-trailing-blank-lines=0`
- **THEN** any trailing blank lines are flagged

### Requirement: Trailing Blank Lines Removal
The tool SHALL remove excessive trailing blank lines.

#### Scenario: Remove extra blank lines
- **WHEN** fixing a file with multiple trailing blank lines
- **THEN** trailing blank lines are reduced to at most one
- **AND** the final newline is preserved

#### Scenario: Strict blank line removal
- **WHEN** fixing with `--max-trailing-blank-lines=0`
- **THEN** all trailing blank lines are removed
- **AND** the final newline is preserved

### Requirement: Trailing Whitespace Detection
The tool SHALL detect trailing whitespace on lines.

#### Scenario: Trailing spaces detection
- **WHEN** a line ends with space characters before the newline
- **THEN** the line is flagged as having trailing whitespace

#### Scenario: Trailing tabs detection
- **WHEN** a line ends with tab characters before the newline
- **THEN** the line is flagged as having trailing whitespace

### Requirement: Trailing Whitespace Removal
The tool SHALL remove trailing whitespace from lines.

#### Scenario: Remove trailing spaces
- **WHEN** fixing a file with trailing whitespace
- **THEN** all trailing spaces and tabs are removed from each line

#### Scenario: Preserve intentional whitespace
- **WHEN** user specifies `--preserve-trailing-whitespace`
- **THEN** trailing whitespace is not removed

#### Scenario: Markdown exception
- **WHEN** processing markdown files
- **AND** a line ends with two spaces (markdown line break)
- **THEN** the two trailing spaces are preserved by default

### Requirement: EOF Report
The tool SHALL report EOF issues clearly.

#### Scenario: Report missing newline
- **WHEN** scanning a file without a final newline
- **THEN** the report shows: "Missing final newline"

#### Scenario: Report trailing blank lines
- **WHEN** scanning a file with excessive trailing blank lines
- **THEN** the report shows: "Excessive trailing blank lines (found 5)"

#### Scenario: Report trailing whitespace
- **WHEN** scanning a file with trailing whitespace
- **THEN** the report shows: "Trailing whitespace on N lines"
