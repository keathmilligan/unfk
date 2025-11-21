# Change: Add EditorConfig Support

## Why

Many projects use `.editorconfig` files as a standardized way to define formatting rules (line endings, indentation, encoding). Currently, unfk only reads its own configuration files. Supporting `.editorconfig` allows unfk to automatically respect existing project conventions without requiring users to duplicate settings, improving adoption and interoperability with editors and other tools.

## What Changes

- Add parsing of `.editorconfig` files using hierarchical discovery (following EditorConfig specification)
- Integrate EditorConfig settings into the configuration precedence hierarchy
- Map EditorConfig properties to unfk settings (end_of_line, indent_style, indent_size, charset, trim_trailing_whitespace, insert_final_newline)
- Add CLI flag `--no-editorconfig` to disable EditorConfig integration when needed

## Impact

- Affected specs: `configuration`
- Affected code: `src/config/mod.rs` (new editorconfig module, config loading logic)
- New dependency: `editorconfig` crate (or similar parser)
- Precedence order becomes: CLI flags > unfk config file > EditorConfig > built-in defaults
