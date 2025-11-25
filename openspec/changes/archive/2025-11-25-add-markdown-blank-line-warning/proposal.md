# Change: Add Markdown Blank Line Warning

## Why
Markdown documents with multiple successive blank lines (double-spacing or more) create unnecessary visual gaps and inconsistent formatting. While a single blank line is standard for paragraph breaks, multiple consecutive blank lines are generally unintentional and should be flagged.

## What Changes
- Add detection for multiple successive blank lines within markdown files (`.md`)
- One blank line between content is allowed; two or more triggers a warning
- Provide optional fix capability to reduce multiple blank lines to a single blank line
- This is a **warning** (unfixable by default) but can be optionally fixed when user explicitly requests it

## Impact
- Affected specs: `eof-repair` (extends blank line handling to cover in-document blank lines for markdown)
- Affected code: `src/analysis/eof.rs` (add successive blank line detection logic)
- New behavior: markdown files will report warnings when multiple consecutive blank lines are detected
- Breaking: No, this is a new warning that doesn't change existing functionality
