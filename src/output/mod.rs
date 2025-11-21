//! Output formatting and reporting

use std::path::Path;

use colored::Colorize;

use crate::analysis::Issue;
use crate::cli::Cli;

/// Reporter handles all output formatting
pub struct Reporter<'a> {
    cli: &'a Cli,
}

impl<'a> Reporter<'a> {
    pub fn new(cli: &'a Cli) -> Self {
        // Disable colors if requested
        if cli.no_color {
            colored::control::set_override(false);
        }

        Self { cli }
    }

    /// Report issues found in a file
    ///
    /// Errors (fixable issues) are shown in red, warnings (unfixable) in yellow.
    pub fn report_file_issues(&self, path: &Path, issues: &[Issue]) {
        if self.cli.quiet {
            return;
        }

        println!("{}", path.display().to_string().bold());
        for issue in issues {
            if issue.is_fixable() {
                // Errors (fixable) - shown in red
                println!(
                    "  {} {}",
                    "error:".red().bold(),
                    issue.description().red()
                );
            } else {
                // Warnings (unfixable) - shown in yellow
                println!(
                    "  {} {}",
                    "warning:".yellow().bold(),
                    issue.description().yellow()
                );
            }
        }
    }

    /// Report that a file has no issues (verbose only)
    pub fn report_file_ok(&self, path: &Path) {
        if !self.cli.verbose || self.cli.quiet {
            return;
        }

        println!("{} {}", "✓".green(), path.display());
    }

    /// Report an error processing a file
    pub fn report_error(&self, path: &Path, error: &anyhow::Error) {
        if self.cli.quiet {
            return;
        }

        eprintln!(
            "{} {}: {}",
            "error:".red().bold(),
            path.display(),
            error
        );
    }

    /// Report what would be fixed (dry-run mode)
    pub fn report_would_fix(&self, path: &Path, issues: &[Issue]) {
        if self.cli.quiet {
            return;
        }

        println!("{} {}", "Would fix:".blue(), path.display());
        if self.cli.verbose {
            for issue in issues {
                println!("  {} {}", "•".yellow(), issue.description());
            }
        }
    }

    /// Report that a file was fixed
    pub fn report_fixed(&self, path: &Path, issues: &[Issue]) {
        if self.cli.quiet {
            return;
        }

        println!("{} {}", "Fixed:".green(), path.display());
        if self.cli.verbose {
            for issue in issues {
                println!("  {} {}", "•".yellow(), issue.description());
            }
        }
    }

    /// Report scan summary with separate error and warning counts
    pub fn report_summary(
        &self,
        files_with_issues: usize,
        error_count: usize,
        warning_count: usize,
    ) {
        if self.cli.quiet {
            return;
        }

        println!();
        let total_issues = error_count + warning_count;
        if total_issues == 0 {
            println!("{}", "No issues found.".green());
        } else {
            // Build summary message with separate error/warning counts
            let mut parts = Vec::new();
            if error_count > 0 {
                parts.push(format!("{} errors", error_count).red().to_string());
            }
            if warning_count > 0 {
                parts.push(format!("{} warnings", warning_count).yellow().to_string());
            }
            println!(
                "Found {} in {} files.",
                parts.join(", "),
                files_with_issues
            );
        }
    }

    /// Report fix summary
    pub fn report_fix_summary(&self, fixed: usize, failed: usize, dry_run: bool) {
        if self.cli.quiet {
            return;
        }

        println!();
        if dry_run {
            if fixed == 0 {
                println!("{}", "No files would be modified.".green());
            } else {
                println!(
                    "{}",
                    format!("Would fix {} files.", fixed).blue()
                );
            }
        } else {
            if fixed > 0 {
                println!(
                    "{}",
                    format!("Fixed {} files.", fixed).green()
                );
            }
            if failed > 0 {
                println!(
                    "{}",
                    format!("Failed to fix {} files.", failed).red()
                );
            }
            if fixed == 0 && failed == 0 {
                println!("{}", "No files needed fixing.".green());
            }
        }
    }
}
