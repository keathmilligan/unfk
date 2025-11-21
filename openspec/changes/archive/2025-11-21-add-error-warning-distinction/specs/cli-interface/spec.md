## ADDED Requirements

### Requirement: Error and Warning Distinction
The CLI SHALL visually distinguish between fixable issues (errors) and unfixable issues (warnings) in scan output.

#### Scenario: Fixable issue displayed as error
- **WHEN** scanning detects an issue that can be fixed by the `fix` command
- **THEN** the issue is displayed in red color
- **AND** the issue is categorized as an "error"

#### Scenario: Unfixable issue displayed as warning
- **WHEN** scanning detects an issue that cannot be automatically fixed
- **THEN** the issue is displayed in yellow color
- **AND** the issue is categorized as a "warning"

#### Scenario: Summary shows separate counts
- **WHEN** scan completes with both errors and warnings
- **THEN** the summary displays separate counts for errors and warnings

## MODIFIED Requirements

### Requirement: Command Structure
The CLI SHALL provide subcommands for scanning and fixing files.

#### Scenario: Scan command reports issues
- **WHEN** user runs `unfk scan <path>`
- **THEN** the tool scans all text files in the path
- **AND** reports fixable issues as errors (red)
- **AND** reports unfixable issues as warnings (yellow)
- **AND** exits with code 0 if no issues, code 1 if issues found

#### Scenario: Fix command repairs issues
- **WHEN** user runs `unfk fix <path>`
- **THEN** the tool scans all text files in the path
- **AND** automatically repairs any formatting issues that are fixable
- **AND** reports what changes were made
- **AND** exits with code 0 if all fixes applied successfully

#### Scenario: Default command is scan
- **WHEN** user runs `unfk <path>` without a subcommand
- **THEN** the tool behaves as if `unfk scan <path>` was invoked
