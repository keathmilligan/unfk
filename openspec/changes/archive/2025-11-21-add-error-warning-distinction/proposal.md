# Change: Add errors and warnings distinction in scan output

## Why
Currently, all detected issues are displayed identically, making it unclear which issues will be automatically fixed by the `fix` command and which require manual intervention. Users need visual differentiation to understand which issues are actionable vs. informational.

## What Changes
- Introduce the concept of "errors" (fixable issues) and "warnings" (unfixable issues)
- Display errors in red to indicate they will be fixed by `fix` command
- Display warnings in yellow to indicate they require manual intervention
- Update summary output to separately count errors and warnings

## Impact
- Affected specs: cli-interface
- Affected code: `src/output/mod.rs`, `src/analysis/mod.rs`
