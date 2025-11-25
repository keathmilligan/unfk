# eof-repair Specification

## Purpose
TBD - created by archiving change add-unfk-cli. Update Purpose after archive.
## Requirements
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

### Requirement: Successive Blank Lines Detection for Markdown
The tool SHALL detect multiple successive blank lines within markdown files.

#### Scenario: Single blank line allowed
- **WHEN** processing a markdown file (`.md`)
- **AND** the file contains a single blank line between content
- **THEN** no issue is reported

#### Scenario: Multiple successive blank lines detected
- **WHEN** processing a markdown file (`.md`)
- **AND** the file contains two or more consecutive blank lines
- **THEN** the file is flagged with a warning about excessive blank lines
- **AND** the warning indicates the number of occurrences

#### Scenario: Non-markdown files ignored
- **WHEN** processing a non-markdown file
- **AND** the file contains multiple consecutive blank lines
- **THEN** no successive blank line warning is reported

#### Scenario: Blank lines at EOF not double-counted
- **WHEN** processing a markdown file with multiple trailing blank lines at end of file
- **THEN** these are reported only as trailing blank line issues (existing behavior)
- **AND** not also reported as successive blank line issues

### Requirement: Successive Blank Lines Repair for Markdown
The tool SHALL optionally reduce multiple successive blank lines to a single blank line in markdown files.

#### Scenario: Reduce multiple blank lines to one
- **WHEN** fixing a markdown file with multiple consecutive blank lines
- **THEN** each sequence of multiple blank lines is reduced to a single blank line
- **AND** content and single blank lines are preserved

#### Scenario: Multiple occurrences handled
- **WHEN** fixing a markdown file with several sequences of multiple blank lines
- **THEN** all sequences are reduced to single blank lines
- **AND** the total number of fixes is reported

#### Scenario: Preserve intentional single blank lines
- **WHEN** fixing a markdown file with single blank lines between paragraphs
- **THEN** these single blank lines are preserved unchanged

### Requirement: Successive Blank Lines Report
The tool SHALL report successive blank line issues clearly for markdown files.

#### Scenario: Report successive blank lines count
- **WHEN** scanning a markdown file with multiple instances of successive blank lines
- **THEN** the report shows: "Multiple successive blank lines (found N occurrences)"
- **AND** the issue is displayed as a warning (yellow)

#### Scenario: Report includes line numbers
- **WHEN** scanning with verbose mode enabled
- **THEN** the report includes the line numbers where successive blank lines occur

