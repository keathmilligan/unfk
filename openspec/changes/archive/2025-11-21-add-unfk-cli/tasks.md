# Tasks: Add unfk CLI Tool

## 1. Project Setup
- [x] 1.1 Initialize Rust project with `cargo init`
- [x] 1.2 Configure Cargo.toml with dependencies (clap, encoding_rs, ignore, toml, etc.)
- [x] 1.3 Set up project structure (src/lib.rs, src/main.rs, modules)
- [ ] 1.4 Configure CI (GitHub Actions for build/test/lint) - deferred
- [ ] 1.5 Add README.md with basic usage documentation - deferred

## 2. CLI Framework
- [x] 2.1 Implement argument parsing with clap (subcommands: scan, fix, init, types, config)
- [x] 2.2 Add global flags (--verbose, --quiet, --config, --no-gitignore)
- [x] 2.3 Add scan/fix specific flags (--line-ending, --indent, --encoding, --dry-run)
- [x] 2.4 Implement exit code handling (0=success, 1=issues, 2=config error, 3=IO error)
- [x] 2.5 Add --help and --version support
- [ ] 2.6 Write CLI integration tests - deferred

## 3. Configuration System
- [x] 3.1 Define configuration schema (Rust structs)
- [x] 3.2 Implement TOML config file parsing
- [x] 3.3 Implement config file discovery (search up directory tree)
- [x] 3.4 Implement configuration hierarchy (CLI > config > defaults)
- [x] 3.5 Add per-pattern rule matching
- [x] 3.6 Implement `unfk init` command
- [x] 3.7 Implement `unfk config --dump` command
- [x] 3.8 Write configuration tests

## 4. File Discovery
- [x] 4.1 Implement directory traversal (using ignore crate)
- [x] 4.2 Implement gitignore pattern matching (using ignore crate)
- [x] 4.3 Implement binary file detection (NULL byte check + magic signatures)
- [x] 4.4 Implement file size limiting
- [x] 4.5 Implement extension filtering (--include, --exclude)
- [x] 4.6 Add ignore pattern support via config
- [x] 4.7 Write file discovery tests

## 5. File Type Detection
- [x] 5.1 Build file extension to type mapping registry
- [x] 5.2 Implement shebang-based type detection
- [x] 5.3 Define default settings per file type (line endings, indentation)
- [x] 5.4 Implement `unfk types` listing command
- [ ] 5.5 Add EditorConfig parsing and integration - deferred
- [x] 5.6 Write file type detection tests

## 6. Line Ending Detection & Repair
- [x] 6.1 Implement line ending detection (LF, CRLF, CR, mixed)
- [x] 6.2 Implement line ending counting/statistics
- [x] 6.3 Implement line ending normalization (convert to target style)
- [x] 6.4 Handle edge cases (empty files, binary-like content)
- [x] 6.5 Write line ending tests

## 7. Indentation Detection & Repair
- [x] 7.1 Implement indentation detection (tabs vs spaces)
- [x] 7.2 Implement indentation width detection
- [x] 7.3 Implement tabs-to-spaces conversion
- [x] 7.4 Implement spaces-to-tabs conversion
- [x] 7.5 Handle Makefile special case (require tabs)
- [x] 7.6 Write indentation tests

## 8. Encoding Detection & Repair
- [x] 8.1 Implement encoding detection (using chardetng)
- [x] 8.2 Implement BOM detection and handling
- [x] 8.3 Implement encoding conversion to UTF-8
- [x] 8.4 Implement invalid character detection
- [ ] 8.5 Implement character replacement option - partial
- [x] 8.6 Write encoding tests

## 9. EOF & Whitespace Repair
- [x] 9.1 Implement final newline detection
- [x] 9.2 Implement final newline insertion
- [x] 9.3 Implement trailing blank line detection
- [x] 9.4 Implement trailing blank line removal
- [x] 9.5 Implement trailing whitespace detection
- [x] 9.6 Implement trailing whitespace removal
- [x] 9.7 Handle markdown trailing spaces exception
- [x] 9.8 Write EOF and whitespace tests

## 10. Analysis Engine
- [x] 10.1 Create Issue enum/struct for all issue types
- [x] 10.2 Implement file analyzer that runs all detectors
- [x] 10.3 Aggregate issues per file and project-wide
- [ ] 10.4 Implement parallel file processing (rayon) - deferred
- [x] 10.5 Write analysis engine tests

## 11. Repair Engine
- [x] 11.1 Create repair plan from detected issues
- [x] 11.2 Implement file repair orchestration
- [x] 11.3 Implement atomic file writes (write to temp, then rename)
- [x] 11.4 Implement dry-run mode (report without modifying)
- [x] 11.5 Write repair engine tests

## 12. Output & Reporting
- [x] 12.1 Implement scan output format (issues per file)
- [x] 12.2 Implement fix output format (changes made)
- [x] 12.3 Implement verbose output mode
- [x] 12.4 Implement quiet mode (exit code only)
- [x] 12.5 Add color support (with --no-color flag)
- [ ] 12.6 Write output formatting tests - deferred

## 13. Integration & Polish
- [x] 13.1 Add .pre-commit-hooks.yaml for pre-commit integration
- [ ] 13.2 Write end-to-end integration tests - deferred
- [ ] 13.3 Benchmark performance on large repos - deferred
- [ ] 13.4 Optimize hot paths if needed - deferred
- [ ] 13.5 Complete documentation (README, --help text) - deferred
- [ ] 13.6 Create release workflow (cross-platform binaries) - deferred

## Dependencies

- Tasks 2, 3, 4, 5 can proceed in parallel after project setup (1)
- Tasks 6, 7, 8, 9 (detection/repair) depend on file discovery (4) and file type detection (5)
- Task 10 (analysis engine) depends on all detection modules (6-9)
- Task 11 (repair engine) depends on analysis engine (10)
- Task 12 (output) depends on analysis engine (10)
- Task 13 (integration) depends on all prior tasks

## Summary

Core implementation is complete. The following items are deferred for future iterations:
- CI/CD configuration (GitHub Actions)
- README documentation
- EditorConfig integration
- Parallel file processing with rayon
- Additional integration/unit tests
- Performance benchmarking
- Release workflow
