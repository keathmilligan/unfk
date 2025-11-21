//! Encoding detection

use crate::analysis::Issue;
use crate::config::FileSettings;

pub struct EncodingAnalyzer<'a> {
    #[allow(dead_code)]
    settings: &'a FileSettings,
}

impl<'a> EncodingAnalyzer<'a> {
    pub fn new(settings: &'a FileSettings) -> Self {
        Self { settings }
    }

    pub fn analyze(&self, content: &[u8]) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Check for UTF-8 BOM
        if content.starts_with(&[0xEF, 0xBB, 0xBF]) {
            issues.push(Issue::Utf8Bom);
        }

        // Check if content is valid UTF-8
        match std::str::from_utf8(content) {
            Ok(_) => {
                // Valid UTF-8, no encoding issues
            }
            Err(_) => {
                // Try to detect encoding
                let detected = self.detect_encoding(content);
                if detected != "utf-8" {
                    issues.push(Issue::NonUtf8Encoding { detected });
                } else {
                    // Invalid UTF-8 sequences
                    let positions = self.find_invalid_utf8_positions(content);
                    if !positions.is_empty() {
                        issues.push(Issue::InvalidUtf8 { positions });
                    }
                }
            }
        }

        issues
    }

    fn detect_encoding(&self, content: &[u8]) -> String {
        // Check for BOM markers
        if content.starts_with(&[0xFF, 0xFE]) {
            return "utf-16le".to_string();
        }
        if content.starts_with(&[0xFE, 0xFF]) {
            return "utf-16be".to_string();
        }

        // Use chardetng for detection
        let mut detector = chardetng::EncodingDetector::new();
        detector.feed(content, true);
        let encoding = detector.guess(None, true);

        encoding.name().to_lowercase()
    }

    fn find_invalid_utf8_positions(&self, content: &[u8]) -> Vec<usize> {
        let mut positions = Vec::new();
        let mut i = 0;

        while i < content.len() {
            let byte = content[i];

            // Determine expected sequence length
            let seq_len = if byte & 0x80 == 0 {
                1 // ASCII
            } else if byte & 0xE0 == 0xC0 {
                2 // 2-byte sequence
            } else if byte & 0xF0 == 0xE0 {
                3 // 3-byte sequence
            } else if byte & 0xF8 == 0xF0 {
                4 // 4-byte sequence
            } else {
                positions.push(i);
                i += 1;
                continue;
            };

            // Check if we have enough bytes
            if i + seq_len > content.len() {
                positions.push(i);
                break;
            }

            // Check continuation bytes
            let mut valid = true;
            for j in 1..seq_len {
                if content[i + j] & 0xC0 != 0x80 {
                    valid = false;
                    break;
                }
            }

            if !valid {
                positions.push(i);
                i += 1;
            } else {
                i += seq_len;
            }
        }

        positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{IndentConfig, LineEnding, TrailingWhitespace};

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
    fn test_valid_utf8() {
        let settings = default_settings();
        let analyzer = EncodingAnalyzer::new(&settings);

        let content = "Hello, world! 你好世界".as_bytes();
        let issues = analyzer.analyze(content);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_utf8_bom() {
        let settings = default_settings();
        let analyzer = EncodingAnalyzer::new(&settings);

        let mut content = vec![0xEF, 0xBB, 0xBF]; // UTF-8 BOM
        content.extend_from_slice(b"Hello");

        let issues = analyzer.analyze(&content);
        assert_eq!(issues.len(), 1);
        assert!(matches!(issues[0], Issue::Utf8Bom));
    }
}
