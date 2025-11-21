//! File analysis and issue detection

mod encoding;
mod eof;
mod indentation;
mod line_endings;

use std::path::Path;

use anyhow::Result;

use crate::config::{Config, FileSettings, LineEnding};
use crate::filetypes::FileTypeRegistry;

pub use encoding::EncodingAnalyzer;
pub use eof::EofAnalyzer;
pub use indentation::IndentationAnalyzer;
pub use line_endings::LineEndingAnalyzer;

/// An issue detected in a file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Issue {
    /// Mixed line endings detected
    MixedLineEndings { lf_count: usize, crlf_count: usize },
    /// Wrong line ending style
    WrongLineEnding {
        expected: LineEndingStyle,
        found: LineEndingStyle,
    },
    /// Mixed indentation (tabs and spaces)
    MixedIndentation { tabs: usize, spaces: usize },
    /// Wrong indentation style
    WrongIndentation {
        expected: IndentationStyle,
        found: IndentationStyle,
    },
    /// Tabs used in file that requires tabs but found spaces in indentation
    TabsRequired,
    /// Non-UTF8 encoding detected
    NonUtf8Encoding { detected: String },
    /// UTF-8 BOM present
    Utf8Bom,
    /// Invalid UTF-8 sequences
    InvalidUtf8 { positions: Vec<usize> },
    /// Missing final newline
    MissingFinalNewline,
    /// Excessive trailing blank lines
    ExcessiveTrailingBlankLines { count: usize },
    /// Trailing whitespace on lines
    TrailingWhitespace { line_count: usize },
}

impl Issue {
    /// Get a human-readable description of the issue
    pub fn description(&self) -> String {
        match self {
            Issue::MixedLineEndings { lf_count, crlf_count } => {
                format!("Mixed line endings ({lf_count} LF, {crlf_count} CRLF)")
            }
            Issue::WrongLineEnding { expected, found } => {
                format!("Wrong line ending (expected {expected:?}, found {found:?})")
            }
            Issue::MixedIndentation { tabs, spaces } => {
                format!("Mixed indentation ({tabs} tabs, {spaces} spaces)")
            }
            Issue::WrongIndentation { expected, found } => {
                format!("Wrong indentation (expected {expected:?}, found {found:?})")
            }
            Issue::TabsRequired => "Tabs required but spaces found".to_string(),
            Issue::NonUtf8Encoding { detected } => {
                format!("Non-UTF8 encoding detected: {detected}")
            }
            Issue::Utf8Bom => "UTF-8 BOM present".to_string(),
            Issue::InvalidUtf8 { positions } => {
                format!("Invalid UTF-8 sequences at {} positions", positions.len())
            }
            Issue::MissingFinalNewline => "Missing final newline".to_string(),
            Issue::ExcessiveTrailingBlankLines { count } => {
                format!("Excessive trailing blank lines ({count})")
            }
            Issue::TrailingWhitespace { line_count } => {
                format!("Trailing whitespace on {line_count} lines")
            }
        }
    }
}

/// Line ending style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEndingStyle {
    Lf,
    Crlf,
    Cr,
    Mixed,
    None,
}

/// Indentation style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndentationStyle {
    Tabs,
    Spaces,
    Mixed,
    None,
}

/// Main analyzer that coordinates all analysis
pub struct Analyzer<'a> {
    config: &'a Config,
    registry: FileTypeRegistry,
}

impl<'a> Analyzer<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            registry: FileTypeRegistry::new(),
        }
    }

    /// Analyze a file and return all detected issues
    pub fn analyze(&self, path: &Path) -> Result<Vec<Issue>> {
        let content = std::fs::read(path)?;
        let settings = self.get_settings(path);
        let file_type = self.registry.detect(path);

        let mut issues = Vec::new();

        // Line ending analysis
        let le_analyzer = LineEndingAnalyzer::new(&settings);
        issues.extend(le_analyzer.analyze(&content));

        // Indentation analysis
        let indent_analyzer = IndentationAnalyzer::new(&settings, file_type);
        issues.extend(indent_analyzer.analyze(&content));

        // Encoding analysis
        let encoding_analyzer = EncodingAnalyzer::new(&settings);
        issues.extend(encoding_analyzer.analyze(&content));

        // EOF analysis
        let eof_analyzer = EofAnalyzer::new(&settings, file_type);
        issues.extend(eof_analyzer.analyze(&content));

        Ok(issues)
    }

    /// Get effective settings for a file
    fn get_settings(&self, path: &Path) -> FileSettings {
        let mut settings = self.config.settings_for_file(path);

        // Apply file type defaults if no explicit config
        if let Some(file_type) = self.registry.detect(path) {
            if settings.line_ending == LineEnding::Auto {
                settings.line_ending = file_type.default_line_ending;
            }
        }

        settings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issue_description() {
        let issue = Issue::MixedLineEndings {
            lf_count: 10,
            crlf_count: 2,
        };
        assert!(issue.description().contains("10 LF"));
        assert!(issue.description().contains("2 CRLF"));
    }
}
