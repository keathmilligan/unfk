# Capability: Encoding Repair

## ADDED Requirements

### Requirement: Encoding Detection
The tool SHALL detect character encoding of text files.

#### Scenario: UTF-8 detection
- **WHEN** a file is encoded in UTF-8 (with or without BOM)
- **THEN** the encoding is detected as UTF-8

#### Scenario: UTF-16 detection
- **WHEN** a file is encoded in UTF-16 (LE or BE)
- **THEN** the encoding is detected as UTF-16

#### Scenario: Latin-1/ISO-8859-1 detection
- **WHEN** a file is encoded in Latin-1/ISO-8859-1
- **THEN** the encoding is detected as Latin-1

#### Scenario: Detection confidence
- **WHEN** encoding cannot be determined with high confidence
- **THEN** the file is flagged for manual review
- **AND** no automatic conversion is performed

### Requirement: Encoding Normalization
The tool SHALL convert files to a target encoding.

#### Scenario: Convert to UTF-8
- **WHEN** fixing a file with `--encoding=utf-8`
- **AND** the file is in a different encoding
- **THEN** the file is converted to UTF-8 without BOM

#### Scenario: UTF-8 BOM handling
- **WHEN** fixing a UTF-8 file that has a BOM
- **AND** BOMs are not desired (default)
- **THEN** the BOM is removed

#### Scenario: Preserve UTF-8 BOM
- **WHEN** fixing with `--preserve-bom`
- **THEN** existing BOMs are not removed

#### Scenario: Add UTF-8 BOM
- **WHEN** fixing with `--add-bom`
- **THEN** a UTF-8 BOM is added if not present

### Requirement: Invalid Character Handling
The tool SHALL handle invalid or unconvertible characters.

#### Scenario: Report invalid UTF-8 sequences
- **WHEN** scanning a file declared as UTF-8 with invalid byte sequences
- **THEN** the invalid sequences are flagged with their byte positions

#### Scenario: Replace invalid characters
- **WHEN** fixing a file with invalid characters
- **AND** `--replace-invalid` is specified
- **THEN** invalid characters are replaced with the Unicode replacement character (U+FFFD)

#### Scenario: Fail on invalid characters
- **WHEN** fixing a file with invalid characters
- **AND** `--replace-invalid` is not specified
- **THEN** the fix fails with an error
- **AND** the file is not modified

### Requirement: Default Encoding
The tool SHALL use UTF-8 as the default target encoding.

#### Scenario: Default to UTF-8
- **WHEN** no encoding is specified
- **THEN** UTF-8 without BOM is the target encoding

#### Scenario: Report non-UTF-8 files
- **WHEN** scanning finds a non-UTF-8 file
- **THEN** the report shows: "Non-UTF-8 encoding detected: Latin-1"

### Requirement: Encoding Report
The tool SHALL report encoding issues clearly.

#### Scenario: Report unexpected encoding
- **WHEN** scanning a file with non-UTF-8 encoding
- **THEN** the report shows the detected encoding

#### Scenario: Report BOM presence
- **WHEN** scanning a file with a BOM
- **THEN** the report notes: "UTF-8 BOM present"

#### Scenario: Report conversion summary
- **WHEN** fixing with encoding changes in verbose mode
- **THEN** the report shows: "Converted from Latin-1 to UTF-8"
