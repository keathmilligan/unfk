# cli-interface Specification

## Purpose
TBD - created by archiving change add-unfk-cli. Update Purpose after archive.
## Requirements
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

### Requirement: Path Arguments
The CLI SHALL accept file and directory paths as arguments.

#### Scenario: Single file processing
- **WHEN** user specifies a single file path
- **THEN** only that file is processed

#### Scenario: Directory processing
- **WHEN** user specifies a directory path
- **THEN** all text files in that directory and subdirectories are processed

#### Scenario: Multiple paths
- **WHEN** user specifies multiple paths
- **THEN** all specified files and directories are processed

#### Scenario: Current directory default
- **WHEN** user runs `unfk` without any path arguments
- **THEN** the current working directory is processed

### Requirement: Override Flags
The CLI SHALL provide flags to override default behavior.

#### Scenario: Line ending override
- **WHEN** user specifies `--line-ending=lf` or `--line-ending=crlf`
- **THEN** the specified line ending style is enforced for all files

#### Scenario: Indentation override
- **WHEN** user specifies `--indent=tabs` or `--indent=spaces:N`
- **THEN** the specified indentation style is enforced for all files

#### Scenario: Encoding override
- **WHEN** user specifies `--encoding=utf-8`
- **THEN** files are converted to the specified encoding

### Requirement: Output Control
The CLI SHALL provide flags to control output verbosity.

#### Scenario: Quiet mode
- **WHEN** user specifies `--quiet` or `-q`
- **THEN** no output is produced
- **AND** only the exit code indicates success or failure

#### Scenario: Verbose mode
- **WHEN** user specifies `--verbose` or `-v`
- **THEN** detailed information about each file is displayed

#### Scenario: Dry run mode
- **WHEN** user specifies `--dry-run` with the fix command
- **THEN** the tool reports what changes would be made
- **AND** no files are modified

### Requirement: Help and Version
The CLI SHALL provide help and version information.

#### Scenario: Help display
- **WHEN** user runs `unfk --help` or `unfk -h`
- **THEN** usage information and available options are displayed

#### Scenario: Subcommand help
- **WHEN** user runs `unfk scan --help`
- **THEN** detailed help for the scan subcommand is displayed

#### Scenario: Version display
- **WHEN** user runs `unfk --version` or `unfk -V`
- **THEN** the tool name and version number are displayed

### Requirement: Exit Codes
The CLI SHALL use distinct exit codes for different outcomes.

#### Scenario: Success exit code
- **WHEN** scanning finds no issues or fixing completes successfully
- **THEN** the tool exits with code 0

#### Scenario: Issues found exit code
- **WHEN** scanning finds formatting issues
- **THEN** the tool exits with code 1

#### Scenario: Configuration error exit code
- **WHEN** invalid arguments or configuration are provided
- **THEN** the tool exits with code 2

#### Scenario: IO error exit code
- **WHEN** a file cannot be read or written due to permissions or IO errors
- **THEN** the tool exits with code 3

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

