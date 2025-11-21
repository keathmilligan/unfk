# Capability: File Scanning

## ADDED Requirements

### Requirement: Directory Traversal
The tool SHALL recursively traverse directories to find text files.

#### Scenario: Recursive scanning
- **WHEN** a directory path is provided
- **THEN** all files in the directory and all subdirectories are examined

#### Scenario: Hidden file handling
- **WHEN** a directory contains hidden files (starting with `.`)
- **THEN** hidden files are included in scanning by default
- **AND** user can exclude them with `--no-hidden`

### Requirement: Gitignore Respect
The tool SHALL respect `.gitignore` patterns by default.

#### Scenario: Gitignore patterns applied
- **WHEN** scanning a Git repository
- **THEN** files matching `.gitignore` patterns are skipped

#### Scenario: Nested gitignore files
- **WHEN** subdirectories contain their own `.gitignore` files
- **THEN** those patterns are applied to files within those subdirectories

#### Scenario: Disable gitignore
- **WHEN** user specifies `--no-gitignore`
- **THEN** `.gitignore` patterns are not applied

### Requirement: Binary File Detection
The tool SHALL detect and skip binary files automatically.

#### Scenario: Binary file skipped
- **WHEN** a file is detected as binary
- **THEN** the file is skipped without processing
- **AND** no warning is displayed unless in verbose mode

#### Scenario: Binary detection method
- **WHEN** checking if a file is binary
- **THEN** the tool checks for NULL bytes in the first 8KB
- **AND** the tool checks for known binary file signatures

#### Scenario: Force binary processing
- **WHEN** user specifies `--include-binary`
- **THEN** detected binary files are processed anyway

### Requirement: File Extension Filtering
The tool SHALL support filtering files by extension.

#### Scenario: Include specific extensions
- **WHEN** user specifies `--include=*.rs,*.md`
- **THEN** only files matching those extensions are processed

#### Scenario: Exclude specific extensions
- **WHEN** user specifies `--exclude=*.min.js,*.lock`
- **THEN** files matching those extensions are skipped

### Requirement: File Size Limits
The tool SHALL skip files exceeding a configurable size limit.

#### Scenario: Default size limit
- **WHEN** a file exceeds 10MB
- **THEN** the file is skipped by default
- **AND** a warning is displayed in verbose mode

#### Scenario: Custom size limit
- **WHEN** user specifies `--max-size=50MB`
- **THEN** the custom size limit is applied

#### Scenario: Disable size limit
- **WHEN** user specifies `--max-size=0`
- **THEN** no size limit is applied

### Requirement: File Type Detection
The tool SHALL detect file types based on extension and content.

#### Scenario: Extension-based detection
- **WHEN** a file has a recognized extension (e.g., `.rs`, `.py`, `.md`)
- **THEN** the file type is determined from the extension

#### Scenario: Shebang detection
- **WHEN** a file has no recognized extension but starts with a shebang line
- **THEN** the file type is inferred from the shebang (e.g., `#!/bin/bash` → shell)

#### Scenario: Fallback to generic text
- **WHEN** file type cannot be determined
- **THEN** the file is treated as generic text
