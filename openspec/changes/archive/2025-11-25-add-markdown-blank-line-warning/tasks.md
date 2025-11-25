# Implementation Tasks

## 1. Analysis Logic
- [x] 1.1 Add successive blank line detection to `src/analysis/eof.rs`
- [x] 1.2 Implement file-type check to target only markdown files (`.md`)
- [x] 1.3 Count consecutive blank lines and flag when count >= 2
- [x] 1.4 Track line numbers where multiple blank lines occur

## 2. Repair Logic
- [x] 2.1 Add successive blank line repair function to `src/repair/mod.rs`
- [x] 2.2 Reduce multiple consecutive blank lines to a single blank line
- [x] 2.3 Preserve single blank lines and content

## 3. CLI Integration
- [x] 3.1 Ensure warnings are displayed in yellow for scan command
- [x] 3.2 Ensure fix command can optionally repair this issue
- [x] 3.3 Update output formatting to show successive blank line warnings

## 4. Testing
- [x] 4.1 Add unit tests for successive blank line detection
- [x] 4.2 Add unit tests for successive blank line repair
- [x] 4.3 Test with real markdown files containing various blank line patterns
- [x] 4.4 Verify other file types are not affected

## 5. Documentation
- [x] 5.1 Update README.md to mention markdown blank line checking
- [x] 5.2 Add example to documentation showing before/after
