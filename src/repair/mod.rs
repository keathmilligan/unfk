//! File repair functionality

use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};

use crate::analysis::{Issue, LineEndingStyle};
use crate::config::{Config, IndentStyle, LineEnding};
use crate::filetypes::FileTypeRegistry;

/// Repairer handles fixing issues in files
pub struct Repairer<'a> {
    config: &'a Config,
    registry: FileTypeRegistry,
}

impl<'a> Repairer<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            registry: FileTypeRegistry::new(),
        }
    }

    /// Repair all issues in a file
    pub fn repair(&self, path: &Path, issues: &[Issue]) -> Result<()> {
        if issues.is_empty() {
            return Ok(());
        }

        let content = std::fs::read(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        let settings = self.config.settings_for_file(path);
        let file_type = self.registry.detect(path);

        // Apply repairs
        let mut repaired = content.clone();

        // Handle encoding first (may affect all other repairs)
        for issue in issues {
            if matches!(issue, Issue::Utf8Bom) {
                repaired = self.remove_bom(&repaired);
            }
        }

        // Convert to string for text-based repairs
        let text = match String::from_utf8(repaired.clone()) {
            Ok(s) => s,
            Err(_) => {
                // Try to decode with detected encoding
                let (decoded, _, _) = encoding_rs::UTF_8.decode(&repaired);
                decoded.into_owned()
            }
        };

        let mut lines: Vec<String> = text.lines().map(|s| s.to_string()).collect();

        // Remove trailing whitespace
        for issue in issues {
            if matches!(issue, Issue::TrailingWhitespace { .. }) {
                let is_markdown = file_type.map(|ft| ft.name == "markdown").unwrap_or(false);
                lines = self.remove_trailing_whitespace(lines, is_markdown);
            }
        }

        // Remove successive blank lines in markdown
        for issue in issues {
            if matches!(issue, Issue::SuccessiveBlankLines { .. }) {
                let is_markdown = file_type.map(|ft| ft.name == "markdown").unwrap_or(false);
                if is_markdown {
                    lines = self.remove_successive_blank_lines(lines);
                }
            }
        }

        // Fix indentation
        for issue in issues {
            match issue {
                Issue::MixedIndentation { .. }
                | Issue::WrongIndentation { .. }
                | Issue::TabsRequired => {
                    lines =
                        self.fix_indentation(lines, &settings.indent.style, settings.indent.width);
                }
                _ => {}
            }
        }

        // Determine target line ending
        let line_ending = match settings.line_ending {
            LineEnding::Lf => "\n",
            LineEnding::Crlf => "\r\n",
            LineEnding::Auto => {
                // Use the dominant style from the file, or LF as default
                let has_crlf_issue = issues.iter().any(|i| {
                    matches!(
                        i,
                        Issue::WrongLineEnding {
                            found: LineEndingStyle::Crlf,
                            ..
                        }
                    )
                });
                let _ = has_crlf_issue; // CRLF files are converted to LF
                "\n"
            }
        };

        // Rebuild content with proper line endings
        let mut result = lines.join(line_ending);

        // Ensure final newline based on config setting
        // This is needed because lines.join() doesn't preserve the original trailing newline
        if settings.final_newline && !result.ends_with('\n') && !result.ends_with("\r\n") {
            result.push_str(line_ending);
        }

        // Remove excessive trailing blank lines
        for issue in issues {
            if matches!(issue, Issue::ExcessiveTrailingBlankLines { .. }) {
                result = self.remove_excessive_trailing_blank_lines(&result, line_ending);
            }
        }

        // Write atomically (write to temp file, then rename)
        self.write_atomic(path, result.as_bytes())?;

        Ok(())
    }

    fn remove_bom(&self, content: &[u8]) -> Vec<u8> {
        if content.starts_with(&[0xEF, 0xBB, 0xBF]) {
            content[3..].to_vec()
        } else {
            content.to_vec()
        }
    }

    fn remove_trailing_whitespace(&self, lines: Vec<String>, is_markdown: bool) -> Vec<String> {
        lines
            .into_iter()
            .map(|line| {
                if is_markdown && line.ends_with("  ") {
                    // Preserve markdown line breaks (double space)
                    let trimmed = line.trim_end();
                    format!("{}  ", trimmed)
                } else {
                    line.trim_end().to_string()
                }
            })
            .collect()
    }

    fn fix_indentation(
        &self,
        lines: Vec<String>,
        target_style: &IndentStyle,
        width: usize,
    ) -> Vec<String> {
        lines
            .into_iter()
            .map(|line| {
                let leading_whitespace: String = line
                    .chars()
                    .take_while(|c| *c == ' ' || *c == '\t')
                    .collect();

                if leading_whitespace.is_empty() {
                    return line;
                }

                let content = &line[leading_whitespace.len()..];

                // Calculate effective width of current indentation
                let effective_width: usize = leading_whitespace
                    .chars()
                    .map(|c| if c == '\t' { width } else { 1 })
                    .sum();

                // Convert to target style
                let new_indent = match target_style {
                    IndentStyle::Tabs => {
                        let tab_count = effective_width / width;
                        let space_remainder = effective_width % width;
                        format!("{}{}", "\t".repeat(tab_count), " ".repeat(space_remainder))
                    }
                    IndentStyle::Spaces => " ".repeat(effective_width),
                };

                format!("{}{}", new_indent, content)
            })
            .collect()
    }

    fn remove_excessive_trailing_blank_lines(&self, content: &str, line_ending: &str) -> String {
        let trimmed = content.trim_end();
        format!("{}{}", trimmed, line_ending)
    }

    fn remove_successive_blank_lines(&self, lines: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut consecutive_blank = 0;

        for line in lines {
            if line.trim().is_empty() {
                consecutive_blank += 1;
                // Allow one blank line, skip additional ones
                if consecutive_blank <= 1 {
                    result.push(line);
                }
            } else {
                consecutive_blank = 0;
                result.push(line);
            }
        }

        result
    }

    fn write_atomic(&self, path: &Path, content: &[u8]) -> Result<()> {
        let parent = path.parent().unwrap_or(Path::new("."));
        let mut temp_file = tempfile::NamedTempFile::new_in(parent)
            .with_context(|| format!("Failed to create temp file in {}", parent.display()))?;

        temp_file
            .write_all(content)
            .with_context(|| "Failed to write to temp file")?;

        temp_file
            .persist(path)
            .with_context(|| format!("Failed to rename temp file to {}", path.display()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn create_config() -> Config {
        Config::default()
    }

    #[test]
    fn test_remove_trailing_whitespace() {
        let config = create_config();
        let repairer = Repairer::new(&config);

        let lines = vec![
            "hello   ".to_string(),
            "world\t".to_string(),
            "no trailing".to_string(),
        ];

        let result = repairer.remove_trailing_whitespace(lines, false);
        assert_eq!(result[0], "hello");
        assert_eq!(result[1], "world");
        assert_eq!(result[2], "no trailing");
    }

    #[test]
    fn test_fix_indentation_tabs_to_spaces() {
        let config = create_config();
        let repairer = Repairer::new(&config);

        let lines = vec!["\thello".to_string(), "\t\tworld".to_string()];

        let result = repairer.fix_indentation(lines, &IndentStyle::Spaces, 4);
        assert_eq!(result[0], "    hello");
        assert_eq!(result[1], "        world");
    }

    #[test]
    fn test_repair_file() {
        let config = create_config();
        let repairer = Repairer::new(&config);

        // Create temp file and get its path, then close it
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        {
            let mut file = std::fs::File::create(&file_path).unwrap();
            writeln!(file, "hello   ").unwrap();
            writeln!(file, "world").unwrap();
        } // File is closed here

        let issues = vec![Issue::TrailingWhitespace { line_count: 1 }];
        repairer.repair(&file_path, &issues).unwrap();

        let content = std::fs::read_to_string(&file_path).unwrap();
        assert!(!content.contains("   "));
    }

    #[test]
    fn test_repair_preserves_final_newline() {
        // Regression test: fixing trailing whitespace should preserve the final newline
        let config = create_config();
        let repairer = Repairer::new(&config);

        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_newline.txt");

        {
            let mut file = std::fs::File::create(&file_path).unwrap();
            // Write file with trailing whitespace BUT with a final newline
            write!(file, "line1   \nline2\n").unwrap();
        }

        // Verify the file has a final newline before repair
        let before = std::fs::read_to_string(&file_path).unwrap();
        assert!(
            before.ends_with('\n'),
            "File should have final newline before repair"
        );

        // Only report trailing whitespace issue (no MissingFinalNewline)
        let issues = vec![Issue::TrailingWhitespace { line_count: 1 }];
        repairer.repair(&file_path, &issues).unwrap();

        let content = std::fs::read_to_string(&file_path).unwrap();
        assert!(
            !content.contains("   "),
            "Trailing whitespace should be removed"
        );
        assert!(
            content.ends_with('\n'),
            "Final newline should be preserved after repair"
        );
    }

    #[test]
    fn test_remove_successive_blank_lines() {
        let config = create_config();
        let repairer = Repairer::new(&config);

        let lines = vec![
            "line1".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "line2".to_string(),
        ];

        let result = repairer.remove_successive_blank_lines(lines);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "line1");
        assert_eq!(result[1], "");
        assert_eq!(result[2], "line2");
    }

    #[test]
    fn test_remove_successive_blank_lines_multiple_sequences() {
        let config = create_config();
        let repairer = Repairer::new(&config);

        let lines = vec![
            "line1".to_string(),
            "".to_string(),
            "".to_string(),
            "line2".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "line3".to_string(),
        ];

        let result = repairer.remove_successive_blank_lines(lines);
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], "line1");
        assert_eq!(result[1], "");
        assert_eq!(result[2], "line2");
        assert_eq!(result[3], "");
        assert_eq!(result[4], "line3");
    }

    #[test]
    fn test_remove_successive_blank_lines_preserves_single() {
        let config = create_config();
        let repairer = Repairer::new(&config);

        let lines = vec!["line1".to_string(), "".to_string(), "line2".to_string()];

        let result = repairer.remove_successive_blank_lines(lines.clone());
        assert_eq!(result.len(), 3);
        assert_eq!(result, lines);
    }

    #[test]
    fn test_repair_successive_blank_lines_in_markdown() {
        let config = create_config();
        let repairer = Repairer::new(&config);

        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.md");

        {
            let mut file = std::fs::File::create(&file_path).unwrap();
            write!(file, "line1\n\n\n\nline2\n").unwrap();
        }

        let issues = vec![Issue::SuccessiveBlankLines { occurrences: 1 }];
        repairer.repair(&file_path, &issues).unwrap();

        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "line1\n\nline2\n");
    }
}
