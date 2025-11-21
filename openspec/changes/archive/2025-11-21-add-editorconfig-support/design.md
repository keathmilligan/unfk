# Design: EditorConfig Support

## Context

EditorConfig is a widely-adopted standard (editorconfig.org) for defining consistent coding styles across editors and IDEs. The spec references EditorConfig integration but it was deferred during initial implementation. This change implements that deferred feature.

The EditorConfig format uses INI-style files with glob patterns for per-file-type settings. Files are discovered hierarchically from the file's directory up to the root (or a file with `root = true`).

## Goals / Non-Goals

**Goals:**
- Parse `.editorconfig` files following the official specification
- Apply EditorConfig settings as a configuration source in the precedence hierarchy
- Support the common properties: `end_of_line`, `indent_style`, `indent_size`, `charset`, `trim_trailing_whitespace`, `insert_final_newline`
- Allow users to disable EditorConfig integration via CLI flag

**Non-Goals:**
- Support EditorConfig properties that don't map to unfk features (e.g., `max_line_length`)
- Write/modify `.editorconfig` files
- Support the `tab_width` property separately from `indent_size` (will treat them equivalently)

## Decisions

### Decision: Use the `ec4rs` crate for parsing
The `ec4rs` crate is a pure Rust implementation of the EditorConfig spec, actively maintained, and avoids requiring external EditorConfig core binaries.

**Alternatives considered:**
- `editorconfig` crate: Wraps the C library, requires system dependency
- Custom parser: Additional maintenance burden, risk of spec non-compliance

### Decision: EditorConfig sits between unfk config and built-in defaults
Precedence order: CLI flags > unfk config file > EditorConfig > built-in defaults

This means:
- Users can always override via CLI or unfk config
- EditorConfig provides sensible project defaults
- Built-in defaults are fallback only

**Rationale:** This matches user expectations—explicit unfk config should win over implicit EditorConfig, but EditorConfig should win over tool defaults.

### Decision: Per-file EditorConfig resolution
EditorConfig settings will be resolved per-file (not once globally) to correctly handle:
- Hierarchical `.editorconfig` files in subdirectories
- Glob patterns that differ by file path

This integrates into `Config::settings_for_file()`.

### Decision: Cache EditorConfig parsing results
Since many files may share the same EditorConfig result, cache parsed results keyed by directory path to avoid repeated file system access and parsing.

## Property Mapping

| EditorConfig Property       | unfk Setting              | Notes                                |
|-----------------------------|---------------------------|--------------------------------------|
| `end_of_line`               | `line_ending`             | lf→Lf, crlf→Crlf                     |
| `indent_style`              | `indent.style`            | tab→Tabs, space→Spaces               |
| `indent_size`               | `indent.width`            | Number or "tab"                      |
| `charset`                   | `encoding`                | utf-8, latin1, etc.                  |
| `trim_trailing_whitespace`  | `trailing_whitespace`     | true→Remove, false→Keep              |
| `insert_final_newline`      | `final_newline`           | Boolean                              |

## Risks / Trade-offs

**Risk:** EditorConfig resolution per-file adds overhead
- **Mitigation:** Cache results by directory; most projects have 1-2 `.editorconfig` files

**Risk:** Subtle behavior differences between unfk and EditorConfig semantics
- **Mitigation:** Document mapping clearly; warn on unmapped properties

**Trade-off:** Adding a new dependency
- The `ec4rs` crate is small, pure Rust, and well-tested—acceptable trade-off for spec compliance

## Open Questions

None at this time.
