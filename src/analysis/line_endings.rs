//! Line ending detection

use crate::analysis::{Issue, LineEndingStyle};
use crate::config::{FileSettings, LineEnding};

pub struct LineEndingAnalyzer<'a> {
    settings: &'a FileSettings,
}

impl<'a> LineEndingAnalyzer<'a> {
    pub fn new(settings: &'a FileSettings) -> Self {
        Self { settings }
    }

    pub fn analyze(&self, content: &[u8]) -> Vec<Issue> {
        let mut issues = Vec::new();

        let stats = self.count_line_endings(content);

        // Check for mixed line endings
        if stats.lf_count > 0 && stats.crlf_count > 0 {
            issues.push(Issue::MixedLineEndings {
                lf_count: stats.lf_count,
                crlf_count: stats.crlf_count,
            });
        } else if stats.cr_count > 0 {
            // Legacy CR-only line endings
            issues.push(Issue::WrongLineEnding {
                expected: self.expected_style(),
                found: LineEndingStyle::Cr,
            });
        } else {
            // Check if line ending style matches expected
            let found = stats.dominant_style();
            let expected = self.expected_style();

            if found != LineEndingStyle::None && found != expected {
                issues.push(Issue::WrongLineEnding { expected, found });
            }
        }

        issues
    }

    fn expected_style(&self) -> LineEndingStyle {
        match self.settings.line_ending {
            LineEnding::Lf => LineEndingStyle::Lf,
            LineEnding::Crlf => LineEndingStyle::Crlf,
            LineEnding::Auto => LineEndingStyle::Lf, // Default to LF for auto
        }
    }

    fn count_line_endings(&self, content: &[u8]) -> LineEndingStats {
        let mut stats = LineEndingStats::default();
        let mut i = 0;

        while i < content.len() {
            if content[i] == b'\r' {
                if i + 1 < content.len() && content[i + 1] == b'\n' {
                    stats.crlf_count += 1;
                    i += 2;
                } else {
                    stats.cr_count += 1;
                    i += 1;
                }
            } else if content[i] == b'\n' {
                stats.lf_count += 1;
                i += 1;
            } else {
                i += 1;
            }
        }

        stats
    }
}

#[derive(Default)]
struct LineEndingStats {
    lf_count: usize,
    crlf_count: usize,
    cr_count: usize,
}

impl LineEndingStats {
    fn dominant_style(&self) -> LineEndingStyle {
        if self.lf_count == 0 && self.crlf_count == 0 && self.cr_count == 0 {
            LineEndingStyle::None
        } else if self.crlf_count > self.lf_count && self.crlf_count > self.cr_count {
            LineEndingStyle::Crlf
        } else if self.lf_count > 0 {
            LineEndingStyle::Lf
        } else if self.cr_count > 0 {
            LineEndingStyle::Cr
        } else {
            LineEndingStyle::None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{IndentConfig, TrailingWhitespace};

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
    fn test_lf_only() {
        let settings = default_settings();
        let analyzer = LineEndingAnalyzer::new(&settings);

        let content = b"line1\nline2\nline3\n";
        let issues = analyzer.analyze(content);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_crlf_when_expecting_lf() {
        let settings = default_settings();
        let analyzer = LineEndingAnalyzer::new(&settings);

        let content = b"line1\r\nline2\r\nline3\r\n";
        let issues = analyzer.analyze(content);
        assert_eq!(issues.len(), 1);
        assert!(matches!(issues[0], Issue::WrongLineEnding { .. }));
    }

    #[test]
    fn test_mixed_line_endings() {
        let settings = default_settings();
        let analyzer = LineEndingAnalyzer::new(&settings);

        let content = b"line1\nline2\r\nline3\n";
        let issues = analyzer.analyze(content);
        assert_eq!(issues.len(), 1);
        assert!(matches!(issues[0], Issue::MixedLineEndings { .. }));
    }
}
