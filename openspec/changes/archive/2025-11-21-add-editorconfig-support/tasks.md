# Tasks: Add EditorConfig Support

## 1. Dependencies and Setup
- [x] 1.1 Add `ec4rs` crate dependency to Cargo.toml

## 2. EditorConfig Module
- [x] 2.1 Create `src/config/editorconfig.rs` module
- [x] 2.2 Implement EditorConfig property parsing and mapping to unfk types
- [x] 2.3 Implement directory-based caching for EditorConfig results
- [x] 2.4 Add unit tests for property mapping

## 3. Configuration Integration
- [x] 3.1 Add `--no-editorconfig` CLI flag to Cli struct
- [x] 3.2 Update `Config::load()` to incorporate EditorConfig in precedence chain
- [x] 3.3 Update `Config::settings_for_file()` to merge EditorConfig settings per-file
- [x] 3.4 Add integration tests for precedence hierarchy

## 4. Testing and Validation
- [x] 4.1 Create test fixtures with sample `.editorconfig` files
- [x] 4.2 Test hierarchical EditorConfig discovery
- [x] 4.3 Test `root = true` behavior
- [x] 4.4 Test `--no-editorconfig` flag
- [x] 4.5 Test precedence: CLI > unfk config > EditorConfig > defaults

## 5. Documentation
- [x] 5.1 Update README with EditorConfig support section
- [x] 5.2 Document property mapping table
- [x] 5.3 Document `--no-editorconfig` flag in CLI help
