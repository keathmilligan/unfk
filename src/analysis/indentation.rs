//! Indentation detection

use crate::analysis::{IndentationStyle, Issue};
use crate::config::{FileSettings, IndentStyle};
use crate::filetypes::FileType;

pub struct IndentationAnalyzer<'a> {
    settings: &'a FileSettings,
    file_type: Option<&'a FileType>,
}

impl<'a> IndentationAnalyzer<'a> {
    pub fn new(settings: &'a FileSettings, file_type: Option<&'a FileType>) -> Self {
        Self {
            settings,
            file_type,
        }
    }

    pub fn analyze(&self, content: &[u8]) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Try to decode as UTF-8 for line analysis
        let text = match std::str::from_utf8(content) {
            Ok(s) => s,
            Err(_) => return issues, // Skip indentation analysis for non-UTF8
        };

        let stats = self.analyze_indentation(text);

        // Check for file-type specific tab requirements
        if let Some(ft) = self.file_type {
            // Tabs required (e.g., Makefiles)
            if ft.tabs_required && stats.space_indented_lines > 0 {
                issues.push(Issue::TabsRequired);
                return issues;
            }
            // Tabs forbidden (e.g., YAML, Python)
            if ft.tabs_forbidden && stats.tab_indented_lines > 0 {
                issues.push(Issue::TabsForbidden);
                return issues;
            }
        }

        // Check for mixed indentation
        if stats.tab_indented_lines > 0 && stats.space_indented_lines > 0 {
            issues.push(Issue::MixedIndentation {
                tabs: stats.tab_indented_lines,
                spaces: stats.space_indented_lines,
            });
        } else {
            // Check if indentation matches expected style
            let found = stats.dominant_style();
            let expected = self.expected_style();

            if found != IndentationStyle::None && found != expected {
                issues.push(Issue::WrongIndentation { expected, found });
            }
        }

        issues
    }

    fn expected_style(&self) -> IndentationStyle {
        match self.settings.indent.style {
            IndentStyle::Tabs => IndentationStyle::Tabs,
            IndentStyle::Spaces => IndentationStyle::Spaces,
        }
    }

    fn analyze_indentation(&self, text: &str) -> IndentationStats {
        let mut stats = IndentationStats::default();

        for line in text.lines() {
            let leading = line.chars().take_while(|c| *c == ' ' || *c == '\t');
            let mut has_tab = false;
            let mut has_space = false;

            for c in leading {
                match c {
                    '\t' => has_tab = true,
                    ' ' => has_space = true,
                    _ => break,
                }
            }

            if has_tab && has_space {
                // Line has mixed indentation
                stats.mixed_lines += 1;
                stats.tab_indented_lines += 1;
                stats.space_indented_lines += 1;
            } else if has_tab {
                stats.tab_indented_lines += 1;
            } else if has_space {
                // Only count as space-indented if it has significant indentation
                let space_count: usize = line.chars().take_while(|c| *c == ' ').count();
                if space_count >= 2 {
                    stats.space_indented_lines += 1;
                }
            }
        }

        stats
    }
}

#[derive(Default)]
struct IndentationStats {
    tab_indented_lines: usize,
    space_indented_lines: usize,
    mixed_lines: usize,
}

impl IndentationStats {
    fn dominant_style(&self) -> IndentationStyle {
        if self.tab_indented_lines == 0 && self.space_indented_lines == 0 {
            IndentationStyle::None
        } else if self.mixed_lines > 0
            || (self.tab_indented_lines > 0 && self.space_indented_lines > 0)
        {
            IndentationStyle::Mixed
        } else if self.tab_indented_lines > 0 {
            IndentationStyle::Tabs
        } else {
            IndentationStyle::Spaces
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{IndentConfig, LineEnding, TrailingWhitespace};

    fn settings_with_spaces() -> FileSettings {
        FileSettings {
            line_ending: LineEnding::Lf,
            indent: IndentConfig {
                style: IndentStyle::Spaces,
                width: 2,
            },
            final_newline: true,
            trailing_whitespace: TrailingWhitespace::Remove,
            encoding: "utf-8".to_string(),
        }
    }

    fn settings_with_tabs() -> FileSettings {
        FileSettings {
            line_ending: LineEnding::Lf,
            indent: IndentConfig {
                style: IndentStyle::Tabs,
                width: 4,
            },
            final_newline: true,
            trailing_whitespace: TrailingWhitespace::Remove,
            encoding: "utf-8".to_string(),
        }
    }

    #[test]
    fn test_spaces_only() {
        let settings = settings_with_spaces();
        let analyzer = IndentationAnalyzer::new(&settings, None);

        let content = b"line1\n  line2\n    line3\n";
        let issues = analyzer.analyze(content);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_tabs_when_expecting_spaces() {
        let settings = settings_with_spaces();
        let analyzer = IndentationAnalyzer::new(&settings, None);

        let content = b"line1\n\tline2\n\t\tline3\n";
        let issues = analyzer.analyze(content);
        assert_eq!(issues.len(), 1);
        assert!(matches!(issues[0], Issue::WrongIndentation { .. }));
    }

    #[test]
    fn test_mixed_indentation() {
        let settings = settings_with_spaces();
        let analyzer = IndentationAnalyzer::new(&settings, None);

        let content = b"line1\n  line2\n\tline3\n";
        let issues = analyzer.analyze(content);
        assert_eq!(issues.len(), 1);
        assert!(matches!(issues[0], Issue::MixedIndentation { .. }));
    }

    #[test]
    fn test_tabs_forbidden_yaml() {
        use crate::filetypes::FileTypeRegistry;
        use std::path::Path;

        let settings = settings_with_spaces();
        let registry = FileTypeRegistry::new();
        let yaml_type = registry.detect(Path::new("test.yaml"));

        let analyzer = IndentationAnalyzer::new(&settings, yaml_type);

        // YAML with tabs should report TabsForbidden
        let content = b"key:\n\tvalue: 1\n";
        let issues = analyzer.analyze(content);
        assert_eq!(issues.len(), 1);
        assert!(matches!(issues[0], Issue::TabsForbidden));
    }

    #[test]
    fn test_tabs_forbidden_python() {
        use crate::filetypes::FileTypeRegistry;
        use std::path::Path;

        let settings = settings_with_spaces();
        let registry = FileTypeRegistry::new();
        let python_type = registry.detect(Path::new("test.py"));

        let analyzer = IndentationAnalyzer::new(&settings, python_type);

        // Python with tabs should report TabsForbidden
        let content = b"def foo():\n\tprint('hello')\n";
        let issues = analyzer.analyze(content);
        assert_eq!(issues.len(), 1);
        assert!(matches!(issues[0], Issue::TabsForbidden));
    }

    #[test]
    fn test_yaml_spaces_ok() {
        use crate::filetypes::FileTypeRegistry;
        use std::path::Path;

        let settings = settings_with_spaces();
        let registry = FileTypeRegistry::new();
        let yaml_type = registry.detect(Path::new("test.yaml"));

        let analyzer = IndentationAnalyzer::new(&settings, yaml_type);

        // YAML with spaces should be fine
        let content = b"key:\n  value: 1\n";
        let issues = analyzer.analyze(content);
        assert!(issues.is_empty());
    }
}
