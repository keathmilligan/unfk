//! unfk - A fast, modern CLI tool for scanning and repairing file formatting issues
//!
//! This library provides the core functionality for detecting and fixing common
//! file formatting problems like mixed line endings, inconsistent indentation,
//! encoding issues, and trailing whitespace.

pub mod analysis;
pub mod cli;
pub mod config;
pub mod discovery;
pub mod filetypes;
pub mod output;
pub mod repair;

/// Exit codes for the CLI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    /// No issues found (scan) or all fixes applied successfully (fix)
    Success = 0,
    /// Issues found (scan) or some files could not be fixed
    IssuesFound = 1,
    /// Configuration or argument error
    ConfigError = 2,
    /// I/O or permission error
    IoError = 3,
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> Self {
        code as i32
    }
}

impl From<ExitCode> for std::process::ExitCode {
    fn from(code: ExitCode) -> Self {
        std::process::ExitCode::from(code as u8)
    }
}
