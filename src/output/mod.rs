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
    pub fn report_file_issues(&self, path: &Path, issues: &[Issue]) {
        if self.cli.quiet {
            return;
        }

        println!("{}", path.display().to_string().red());
        for issue in issues {
            println!("  {} {}", "•".yellow(), issue.description());
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

    /// Report scan summary
    pub fn report_summary(&self, files_with_issues: usize, total_issues: usize) {
        if self.cli.quiet {
            return;
        }

        println!();
        if total_issues == 0 {
            println!("{}", "No issues found.".green());
        } else {
            println!(
                "{}",
                format!(
                    "Found {} issues in {} files.",
                    total_issues, files_with_issues
                )
                .red()
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
