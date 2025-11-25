# eof-repair Specification Deltas

## ADDED Requirements

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
