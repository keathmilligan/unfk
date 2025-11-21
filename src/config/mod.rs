//! Configuration management

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::{Deserialize, Serialize};

use crate::cli::Cli;

/// Main configuration struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Path to the config file that was loaded (if any)
    #[serde(skip)]
    pub source_path: Option<PathBuf>,

    /// Default line ending style
    #[serde(rename = "line-ending")]
    pub line_ending: LineEnding,

    /// Target encoding
    pub encoding: String,

    /// Ensure files end with a newline
    #[serde(rename = "final-newline")]
    pub final_newline: bool,

    /// How to handle trailing whitespace
    #[serde(rename = "trailing-whitespace")]
    pub trailing_whitespace: TrailingWhitespace,

    /// Indentation settings
    pub indent: IndentConfig,

    /// Patterns to ignore
    #[serde(default)]
    pub ignore: Vec<String>,

    /// Per-pattern rules
    #[serde(default)]
    pub rules: Vec<Rule>,

    /// Maximum file size in bytes (default 10MB)
    #[serde(rename = "max-file-size")]
    pub max_file_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            source_path: None,
            line_ending: LineEnding::Lf,
            encoding: "utf-8".to_string(),
            final_newline: true,
            trailing_whitespace: TrailingWhitespace::Remove,
            indent: IndentConfig::default(),
            ignore: vec![
                "node_modules".to_string(),
                "target".to_string(),
                ".git".to_string(),
                "*.min.js".to_string(),
                "*.min.css".to_string(),
            ],
            rules: Vec::new(),
            max_file_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

impl Config {
    /// Load configuration from file and CLI overrides
    pub fn load(cli: &Cli) -> Result<Self> {
        let mut config = if let Some(path) = &cli.config {
            Self::load_from_file(path)?
        } else {
            Self::discover()?
        };

        // Apply CLI overrides
        config.apply_cli_overrides(cli);

        Ok(config)
    }

    /// Load configuration from a specific file
    fn load_from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let mut config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        config.source_path = Some(path.to_path_buf());
        Ok(config)
    }

    /// Discover configuration file by searching up the directory tree
    fn discover() -> Result<Self> {
        let current_dir = std::env::current_dir()?;
        let config_names = [".unfkrc.toml", ".unfkrc", "unfk.toml"];

        let mut dir = current_dir.as_path();
        loop {
            for name in &config_names {
                let config_path = dir.join(name);
                if config_path.exists() {
                    return Self::load_from_file(&config_path);
                }
            }

            match dir.parent() {
                Some(parent) => dir = parent,
                None => break,
            }
        }

        // No config file found, use defaults
        Ok(Self::default())
    }

    /// Apply CLI flag overrides to the configuration
    fn apply_cli_overrides(&mut self, cli: &Cli) {
        if let Some(line_ending) = &cli.line_ending {
            self.line_ending = match line_ending {
                crate::cli::LineEndingArg::Lf => LineEnding::Lf,
                crate::cli::LineEndingArg::Crlf => LineEnding::Crlf,
                crate::cli::LineEndingArg::Auto => LineEnding::Auto,
            };
        }

        if let Some(indent) = &cli.indent {
            self.indent = IndentConfig::parse(indent);
        }

        if let Some(encoding) = &cli.encoding {
            self.encoding = encoding.clone();
        }

        if let Some(max_size) = &cli.max_size {
            if let Some(size) = parse_size(max_size) {
                self.max_file_size = size;
            }
        }

        // Add CLI excludes to ignore list
        self.ignore.extend(cli.exclude.clone());
    }

    /// Build a GlobSet from the ignore patterns
    pub fn build_ignore_globset(&self) -> Result<GlobSet> {
        let mut builder = GlobSetBuilder::new();
        for pattern in &self.ignore {
            builder.add(Glob::new(pattern)?);
        }
        Ok(builder.build()?)
    }

    /// Build a GlobSet from include patterns
    pub fn build_include_globset(&self, patterns: &[String]) -> Result<Option<GlobSet>> {
        if patterns.is_empty() {
            return Ok(None);
        }
        let mut builder = GlobSetBuilder::new();
        for pattern in patterns {
            builder.add(Glob::new(pattern)?);
        }
        Ok(Some(builder.build()?))
    }

    /// Get the effective settings for a specific file
    pub fn settings_for_file(&self, path: &Path) -> FileSettings {
        let mut settings = FileSettings {
            line_ending: self.line_ending,
            indent: self.indent.clone(),
            final_newline: self.final_newline,
            trailing_whitespace: self.trailing_whitespace,
            encoding: self.encoding.clone(),
        };

        // Apply per-pattern rules
        let path_str = path.to_string_lossy();
        for rule in &self.rules {
            if rule.matches(&path_str) {
                rule.apply_to(&mut settings);
            }
        }

        settings
    }
}

/// Line ending style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LineEnding {
    /// Unix-style LF
    #[default]
    Lf,
    /// Windows-style CRLF
    Crlf,
    /// Auto-detect from file
    Auto,
}

/// Trailing whitespace handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TrailingWhitespace {
    /// Remove trailing whitespace
    #[default]
    Remove,
    /// Keep trailing whitespace
    Keep,
}

/// Indentation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IndentConfig {
    /// Style: tabs or spaces
    pub style: IndentStyle,
    /// Width in spaces (for space indentation or tab display width)
    pub width: usize,
}

impl Default for IndentConfig {
    fn default() -> Self {
        Self {
            style: IndentStyle::Spaces,
            width: 2,
        }
    }
}

impl IndentConfig {
    /// Parse an indent string like "tabs" or "spaces:4"
    pub fn parse(s: &str) -> Self {
        if s == "tabs" {
            Self {
                style: IndentStyle::Tabs,
                width: 4,
            }
        } else if let Some(width_str) = s.strip_prefix("spaces:") {
            let width = width_str.parse().unwrap_or(2);
            Self {
                style: IndentStyle::Spaces,
                width,
            }
        } else if s == "spaces" {
            Self {
                style: IndentStyle::Spaces,
                width: 2,
            }
        } else {
            Self::default()
        }
    }
}

/// Indentation style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum IndentStyle {
    /// Use tabs
    Tabs,
    /// Use spaces
    #[default]
    Spaces,
}

/// Per-pattern rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// Glob pattern to match
    pub pattern: String,

    /// Line ending override
    #[serde(rename = "line-ending")]
    pub line_ending: Option<LineEnding>,

    /// Indentation override
    pub indent: Option<IndentConfig>,

    /// Final newline override
    #[serde(rename = "final-newline")]
    pub final_newline: Option<bool>,

    /// Trailing whitespace override
    #[serde(rename = "trailing-whitespace")]
    pub trailing_whitespace: Option<TrailingWhitespace>,
}

impl Rule {
    /// Check if this rule matches a path
    pub fn matches(&self, path: &str) -> bool {
        if let Ok(glob) = Glob::new(&self.pattern) {
            glob.compile_matcher().is_match(path)
        } else {
            false
        }
    }

    /// Apply this rule's overrides to file settings
    pub fn apply_to(&self, settings: &mut FileSettings) {
        if let Some(le) = self.line_ending {
            settings.line_ending = le;
        }
        if let Some(indent) = &self.indent {
            settings.indent = indent.clone();
        }
        if let Some(fnl) = self.final_newline {
            settings.final_newline = fnl;
        }
        if let Some(tw) = self.trailing_whitespace {
            settings.trailing_whitespace = tw;
        }
    }
}

/// Effective settings for a specific file
#[derive(Debug, Clone)]
pub struct FileSettings {
    pub line_ending: LineEnding,
    pub indent: IndentConfig,
    pub final_newline: bool,
    pub trailing_whitespace: TrailingWhitespace,
    pub encoding: String,
}

/// Parse a size string like "10MB" or "1024"
fn parse_size(s: &str) -> Option<usize> {
    let s = s.trim().to_uppercase();
    if let Some(num) = s.strip_suffix("GB") {
        num.trim().parse::<usize>().ok().map(|n| n * 1024 * 1024 * 1024)
    } else if let Some(num) = s.strip_suffix("MB") {
        num.trim().parse::<usize>().ok().map(|n| n * 1024 * 1024)
    } else if let Some(num) = s.strip_suffix("KB") {
        num.trim().parse::<usize>().ok().map(|n| n * 1024)
    } else if let Some(num) = s.strip_suffix('B') {
        num.trim().parse().ok()
    } else {
        s.parse().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size() {
        assert_eq!(parse_size("1024"), Some(1024));
        assert_eq!(parse_size("10KB"), Some(10 * 1024));
        assert_eq!(parse_size("10MB"), Some(10 * 1024 * 1024));
        assert_eq!(parse_size("1GB"), Some(1024 * 1024 * 1024));
    }

    #[test]
    fn test_indent_config_parse() {
        let tabs = IndentConfig::parse("tabs");
        assert_eq!(tabs.style, IndentStyle::Tabs);

        let spaces4 = IndentConfig::parse("spaces:4");
        assert_eq!(spaces4.style, IndentStyle::Spaces);
        assert_eq!(spaces4.width, 4);
    }
}
