//! End-of-file and trailing whitespace detection

use crate::analysis::Issue;
use crate::config::{FileSettings, TrailingWhitespace};
use crate::filetypes::FileType;

pub struct EofAnalyzer<'a> {
    settings: &'a FileSettings,
    file_type: Option<&'a FileType>,
}

impl<'a> EofAnalyzer<'a> {
    pub fn new(settings: &'a FileSettings, file_type: Option<&'a FileType>) -> Self {
        Self {
            settings,
            file_type,
        }
    }

    pub fn analyze(&self, content: &[u8]) -> Vec<Issue> {
        let mut issues = Vec::new();

        if content.is_empty() {
            return issues;
        }

        // Check for missing final newline
        if self.settings.final_newline {
            let ends_with_newline = content.ends_with(b"\n") || content.ends_with(b"\r\n");
            if !ends_with_newline {
                issues.push(Issue::MissingFinalNewline);
            }
        }

        // Check for excessive trailing blank lines
        let trailing_blank_lines = self.count_trailing_blank_lines(content);
        if trailing_blank_lines > 1 {
            issues.push(Issue::ExcessiveTrailingBlankLines {
                count: trailing_blank_lines,
            });
        }

        // Check for trailing whitespace
        if self.settings.trailing_whitespace == TrailingWhitespace::Remove {
            let lines_with_trailing = self.count_trailing_whitespace_lines(content);
            if lines_with_trailing > 0 {
                // Check for markdown exception (trailing double space)
                let is_markdown = self
                    .file_type
                    .map(|ft| ft.name == "markdown")
                    .unwrap_or(false);

                if !is_markdown || !self.is_only_markdown_line_breaks(content) {
                    issues.push(Issue::TrailingWhitespace {
                        line_count: lines_with_trailing,
                    });
                }
            }
        }

        // Check for successive blank lines in markdown files
        let is_markdown = self
            .file_type
            .map(|ft| ft.name == "markdown")
            .unwrap_or(false);

        if is_markdown {
            let successive_blank_lines = self.count_successive_blank_lines(content);
            if successive_blank_lines > 0 {
                issues.push(Issue::SuccessiveBlankLines {
                    occurrences: successive_blank_lines,
                });
            }
        }

        issues
    }

    fn count_trailing_blank_lines(&self, content: &[u8]) -> usize {
        let text = match std::str::from_utf8(content) {
            Ok(s) => s,
            Err(_) => return 0,
        };

        let mut count = 0;
        for line in text.lines().rev() {
            if line.trim().is_empty() {
                count += 1;
            } else {
                break;
            }
        }

        count
    }

    fn count_trailing_whitespace_lines(&self, content: &[u8]) -> usize {
        let text = match std::str::from_utf8(content) {
            Ok(s) => s,
            Err(_) => return 0,
        };

        let mut count = 0;
        for line in text.lines() {
            if line.ends_with(' ') || line.ends_with('\t') {
                count += 1;
            }
        }

        count
    }

    fn is_only_markdown_line_breaks(&self, content: &[u8]) -> bool {
        let text = match std::str::from_utf8(content) {
            Ok(s) => s,
            Err(_) => return false,
        };

        // Check if all trailing whitespace is exactly 2 spaces (markdown line break)
        for line in text.lines() {
            if line.ends_with(' ') || line.ends_with('\t') {
                let trimmed = line.trim_end();
                let whitespace: String = line[trimmed.len()..].to_string();
                if whitespace != "  " {
                    return false;
                }
            }
        }

        true
    }

    fn count_successive_blank_lines(&self, content: &[u8]) -> usize {
        let text = match std::str::from_utf8(content) {
            Ok(s) => s,
            Err(_) => return 0,
        };

        let lines: Vec<&str> = text.lines().collect();
        let mut occurrences = 0;
        let mut consecutive_blank = 0;

        // Don't count trailing blank lines (they're handled separately)
        let non_trailing_lines = {
            let mut end_idx = lines.len();
            for line in lines.iter().rev() {
                if line.trim().is_empty() {
                    end_idx -= 1;
                } else {
                    break;
                }
            }
            end_idx
        };

        for i in 0..non_trailing_lines {
            if lines[i].trim().is_empty() {
                consecutive_blank += 1;
            } else {
                if consecutive_blank >= 2 {
                    occurrences += 1;
                }
                consecutive_blank = 0;
            }
        }

        // Check if we ended on a sequence of blank lines (but not trailing)
        if consecutive_blank >= 2 {
            occurrences += 1;
        }

        occurrences
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{IndentConfig, IndentStyle, LineEnding};

    fn default_settings() -> FileSettings {
        FileSettings {
            line_ending: LineEnding::Lf,
            indent: IndentConfig::default(),
            final_newline: true,
            trailing_whitespace: TrailingWhitespace::Remove,
            encoding: "utf-8".to_string(),
        }
    }

    fn markdown_filetype() -> FileType {
        FileType {
            name: "markdown".to_string(),
            extensions: vec!["md"],
            shebangs: vec![],
            default_line_ending: LineEnding::Lf,
            default_indent: IndentConfig {
                style: IndentStyle::Spaces,
                width: 2,
            },
            tabs_required: false,
            tabs_forbidden: false,
        }
    }

    #[test]
    fn test_missing_final_newline() {
        let settings = default_settings();
        let analyzer = EofAnalyzer::new(&settings, None);

        let content = b"line1\nline2";
        let issues = analyzer.analyze(content);
        assert_eq!(issues.len(), 1);
        assert!(matches!(issues[0], Issue::MissingFinalNewline));
    }

    #[test]
    fn test_proper_final_newline() {
        let settings = default_settings();
        let analyzer = EofAnalyzer::new(&settings, None);

        let content = b"line1\nline2\n";
        let issues = analyzer.analyze(content);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_excessive_trailing_blank_lines() {
        let settings = default_settings();
        let analyzer = EofAnalyzer::new(&settings, None);

        let content = b"line1\nline2\n\n\n\n";
        let issues = analyzer.analyze(content);
        assert!(issues
            .iter()
            .any(|i| matches!(i, Issue::ExcessiveTrailingBlankLines { .. })));
    }

    #[test]
    fn test_trailing_whitespace() {
        let settings = default_settings();
        let analyzer = EofAnalyzer::new(&settings, None);

        let content = b"line1   \nline2\t\nline3\n";
        let issues = analyzer.analyze(content);
        assert!(issues
            .iter()
            .any(|i| matches!(i, Issue::TrailingWhitespace { line_count: 2 })));
    }

    #[test]
    fn test_successive_blank_lines_single_allowed() {
        let settings = default_settings();
        let markdown_type = markdown_filetype();
        let analyzer = EofAnalyzer::new(&settings, Some(&markdown_type));

        let content = b"line1\n\nline2\n";
        let issues = analyzer.analyze(content);
        assert!(!issues
            .iter()
            .any(|i| matches!(i, Issue::SuccessiveBlankLines { .. })));
    }

    #[test]
    fn test_successive_blank_lines_multiple_detected() {
        let settings = default_settings();
        let markdown_type = markdown_filetype();
        let analyzer = EofAnalyzer::new(&settings, Some(&markdown_type));

        let content = b"line1\n\n\nline2\n";
        let issues = analyzer.analyze(content);
        assert!(issues
            .iter()
            .any(|i| matches!(i, Issue::SuccessiveBlankLines { occurrences: 1 })));
    }

    #[test]
    fn test_successive_blank_lines_multiple_occurrences() {
        let settings = default_settings();
        let markdown_type = markdown_filetype();
        let analyzer = EofAnalyzer::new(&settings, Some(&markdown_type));

        let content = b"line1\n\n\nline2\n\n\n\nline3\n";
        let issues = analyzer.analyze(content);
        assert!(issues
            .iter()
            .any(|i| matches!(i, Issue::SuccessiveBlankLines { occurrences: 2 })));
    }

    #[test]
    fn test_successive_blank_lines_non_markdown_ignored() {
        let settings = default_settings();
        let analyzer = EofAnalyzer::new(&settings, None);

        let content = b"line1\n\n\nline2\n";
        let issues = analyzer.analyze(content);
        assert!(!issues
            .iter()
            .any(|i| matches!(i, Issue::SuccessiveBlankLines { .. })));
    }

    #[test]
    fn test_successive_blank_lines_trailing_not_counted() {
        let settings = default_settings();
        let markdown_type = markdown_filetype();
        let analyzer = EofAnalyzer::new(&settings, Some(&markdown_type));

        // Multiple blank lines at EOF should not be counted as successive blank lines
        let content = b"line1\nline2\n\n\n\n";
        let issues = analyzer.analyze(content);
        // Should have trailing blank lines issue but not successive blank lines
        assert!(issues
            .iter()
            .any(|i| matches!(i, Issue::ExcessiveTrailingBlankLines { .. })));
        assert!(!issues
            .iter()
            .any(|i| matches!(i, Issue::SuccessiveBlankLines { .. })));
    }
}
