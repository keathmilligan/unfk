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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{IndentConfig, LineEnding};

    fn default_settings() -> FileSettings {
        FileSettings {
            line_ending: LineEnding::Lf,
            indent: IndentConfig::default(),
            final_newline: true,
            trailing_whitespace: TrailingWhitespace::Remove,
            encoding: "utf-8".to_string(),
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
}
