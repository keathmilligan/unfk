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
    /// Tabs used in file where tabs are forbidden (YAML, Python)
    TabsForbidden,
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
    /// Multiple successive blank lines in markdown
    SuccessiveBlankLines { occurrences: usize },
}

impl Issue {
    /// Check if this issue can be automatically fixed by the `fix` command
    pub fn is_fixable(&self) -> bool {
        match self {
            // These issues cannot be automatically fixed (warnings)
            Issue::TabsForbidden => false,
            Issue::NonUtf8Encoding { .. } => false,
            Issue::InvalidUtf8 { .. } => false,
            Issue::MissingFinalNewline => false,
            Issue::TrailingWhitespace { .. } => false,
            Issue::SuccessiveBlankLines { .. } => false,
            // All other issues can be fixed (errors)
            _ => true,
        }
    }

    /// Get a human-readable description of the issue
    pub fn description(&self) -> String {
        match self {
            Issue::MixedLineEndings { lf_count, crlf_count } => {
                format!("Mixed line endings ({lf_count} LF, {crlf_count} CRLF)")
            }
            Issue::WrongLineEnding { expected, found } => {
                format!("Wrong line ending (expected {expected}, found {found})")
            }
            Issue::MixedIndentation { tabs, spaces } => {
                format!("Mixed indentation ({tabs} tabs, {spaces} spaces)")
            }
            Issue::WrongIndentation { expected, found } => {
                format!("Wrong indentation (expected {expected:?}, found {found:?})")
            }
            Issue::TabsRequired => "Tabs required but spaces found".to_string(),
            Issue::TabsForbidden => "Tabs forbidden but tabs found in indentation".to_string(),
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
            Issue::SuccessiveBlankLines { occurrences } => {
                format!("Multiple successive blank lines (found {occurrences} occurrences)")
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

impl std::fmt::Display for LineEndingStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineEndingStyle::Lf => write!(f, "LF"),
            LineEndingStyle::Crlf => write!(f, "CRLF"),
            LineEndingStyle::Cr => write!(f, "CR"),
            LineEndingStyle::Mixed => write!(f, "Mixed"),
            LineEndingStyle::None => write!(f, "None"),
        }
    }
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

        // Apply file type defaults if no explicit config override exists
        if let Some(file_type) = self.registry.detect(path) {
            // Apply line ending default when set to Auto
            if settings.line_ending == LineEnding::Auto {
                settings.line_ending = file_type.default_line_ending;
            }

            // Apply indentation default if no explicit per-file rule matched
            // We check if the settings still match the global defaults
            if !self.has_indent_override_for_path(path) {
                settings.indent = file_type.default_indent.clone();
            }
        }

        settings
    }

    /// Check if there's an explicit indent override rule for this path
    fn has_indent_override_for_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        for rule in &self.config.rules {
            if rule.indent.is_some() && rule.matches(&path_str) {
                return true;
            }
        }
        false
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

    #[test]
    fn test_is_fixable_returns_true_for_fixable_issues() {
        // All these issues should be fixable (errors)
        let fixable_issues = vec![
            Issue::MixedLineEndings {
                lf_count: 10,
                crlf_count: 2,
            },
            Issue::WrongLineEnding {
                expected: LineEndingStyle::Lf,
                found: LineEndingStyle::Crlf,
            },
            Issue::MixedIndentation { tabs: 5, spaces: 3 },
            Issue::WrongIndentation {
                expected: IndentationStyle::Spaces,
                found: IndentationStyle::Tabs,
            },
            Issue::TabsRequired,
            Issue::Utf8Bom,
            Issue::ExcessiveTrailingBlankLines { count: 3 },
        ];

        for issue in fixable_issues {
            assert!(
                issue.is_fixable(),
                "{:?} should be fixable",
                issue
            );
        }
    }

    #[test]
    fn test_is_fixable_returns_false_for_unfixable_issues() {
        // These issues cannot be automatically fixed (warnings)
        let unfixable_issues = vec![
            Issue::TabsForbidden,
            Issue::NonUtf8Encoding {
                detected: "windows-1252".to_string(),
            },
            Issue::InvalidUtf8 {
                positions: vec![10, 20, 30],
            },
            Issue::MissingFinalNewline,
            Issue::TrailingWhitespace { line_count: 5 },
            Issue::SuccessiveBlankLines { occurrences: 3 },
        ];

        for issue in unfixable_issues {
            assert!(
                !issue.is_fixable(),
                "{:?} should NOT be fixable",
                issue
            );
        }
    }
}
