//! EditorConfig integration for unfk
//!
//! This module provides parsing and mapping of `.editorconfig` files
//! to unfk's configuration types.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

use ec4rs::property::{EndOfLine, IndentSize, IndentStyle as EcIndentStyle};
use ec4rs::Properties;

use super::{IndentConfig, IndentStyle, LineEnding, TrailingWhitespace};

/// Cached EditorConfig properties by directory path
static CACHE: Mutex<Option<HashMap<String, EditorConfigSettings>>> = Mutex::new(None);

/// Settings extracted from EditorConfig that map to unfk settings
#[derive(Debug, Clone, Default)]
pub struct EditorConfigSettings {
    pub line_ending: Option<LineEnding>,
    pub indent_style: Option<IndentStyle>,
    pub indent_size: Option<usize>,
    pub charset: Option<String>,
    pub trim_trailing_whitespace: Option<TrailingWhitespace>,
    pub insert_final_newline: Option<bool>,
}

impl EditorConfigSettings {
    /// Get EditorConfig settings for a specific file path
    ///
    /// This uses the ec4rs crate to resolve the hierarchical `.editorconfig`
    /// files and extract the properties for the given file.
    pub fn for_file(path: &Path) -> Self {
        // Try to get from cache first
        if let Some(cached) = Self::get_cached(path) {
            return cached;
        }

        // Resolve properties using ec4rs
        let settings = Self::resolve(path);

        // Cache the result
        Self::cache(path, settings.clone());

        settings
    }

    /// Resolve EditorConfig properties for a file
    fn resolve(path: &Path) -> Self {
        let properties = match ec4rs::properties_of(path) {
            Ok(props) => props,
            Err(_) => return Self::default(),
        };

        Self::from_properties(&properties)
    }

    /// Convert ec4rs Properties to our settings
    fn from_properties(props: &Properties) -> Self {
        let mut settings = Self::default();

        // Map end_of_line
        if let Ok(eol) = props.get::<EndOfLine>() {
            settings.line_ending = Some(match eol {
                EndOfLine::Lf => LineEnding::Lf,
                EndOfLine::CrLf => LineEnding::Crlf,
                EndOfLine::Cr => LineEnding::Lf, // CR-only not supported, fall back to LF
            });
        }

        // Map indent_style
        if let Ok(style) = props.get::<EcIndentStyle>() {
            settings.indent_style = Some(match style {
                EcIndentStyle::Tabs => IndentStyle::Tabs,
                EcIndentStyle::Spaces => IndentStyle::Spaces,
            });
        }

        // Map indent_size
        if let Ok(size) = props.get::<IndentSize>() {
            settings.indent_size = match size {
                IndentSize::Value(n) => Some(n),
                IndentSize::UseTabWidth => {
                    // Fall back to tab_width or default
                    props
                        .get::<ec4rs::property::TabWidth>()
                        .ok()
                        .map(|tw| match tw {
                            ec4rs::property::TabWidth::Value(n) => n,
                        })
                        .or(Some(4))
                }
            };
        }

        // Map charset
        if let Ok(charset) = props.get::<ec4rs::property::Charset>() {
            settings.charset = Some(match charset {
                ec4rs::property::Charset::Latin1 => "latin1".to_string(),
                ec4rs::property::Charset::Utf8 => "utf-8".to_string(),
                ec4rs::property::Charset::Utf8Bom => "utf-8-bom".to_string(),
                ec4rs::property::Charset::Utf16Be => "utf-16be".to_string(),
                ec4rs::property::Charset::Utf16Le => "utf-16le".to_string(),
            });
        }

        // Map trim_trailing_whitespace
        if let Ok(ec4rs::property::TrimTrailingWs::Value(v)) =
            props.get::<ec4rs::property::TrimTrailingWs>()
        {
            settings.trim_trailing_whitespace = Some(if v {
                TrailingWhitespace::Remove
            } else {
                TrailingWhitespace::Keep
            });
        }

        // Map insert_final_newline
        if let Ok(ec4rs::property::FinalNewline::Value(v)) =
            props.get::<ec4rs::property::FinalNewline>()
        {
            settings.insert_final_newline = Some(v);
        }

        settings
    }

    /// Try to get cached settings for a file
    fn get_cached(path: &Path) -> Option<Self> {
        let key = Self::cache_key(path)?;
        let cache = CACHE.lock().ok()?;
        cache.as_ref()?.get(&key).cloned()
    }

    /// Cache settings for a file
    fn cache(path: &Path, settings: Self) {
        if let Some(key) = Self::cache_key(path) {
            if let Ok(mut cache) = CACHE.lock() {
                let map = cache.get_or_insert_with(HashMap::new);
                map.insert(key, settings);
            }
        }
    }

    /// Get cache key for a path (uses parent directory)
    fn cache_key(path: &Path) -> Option<String> {
        // Use the file's absolute path as key since editorconfig
        // resolution depends on the full path hierarchy
        path.canonicalize()
            .ok()
            .map(|p| p.to_string_lossy().to_string())
    }

    /// Clear the cache (useful for testing)
    #[allow(dead_code)]
    pub fn clear_cache() {
        if let Ok(mut cache) = CACHE.lock() {
            *cache = None;
        }
    }

    /// Build an IndentConfig from the EditorConfig settings
    pub fn to_indent_config(&self) -> Option<IndentConfig> {
        match (self.indent_style, self.indent_size) {
            (Some(style), Some(width)) => Some(IndentConfig { style, width }),
            (Some(style), None) => Some(IndentConfig {
                style,
                width: if style == IndentStyle::Tabs { 4 } else { 2 },
            }),
            (None, Some(width)) => Some(IndentConfig {
                style: IndentStyle::Spaces,
                width,
            }),
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_editorconfig(dir: &Path, content: &str) {
        fs::write(dir.join(".editorconfig"), content).unwrap();
    }

    #[test]
    fn test_line_ending_mapping() {
        let temp = TempDir::new().unwrap();
        create_editorconfig(
            temp.path(),
            r#"
root = true

[*]
end_of_line = lf
"#,
        );

        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, "").unwrap();

        EditorConfigSettings::clear_cache();
        let settings = EditorConfigSettings::for_file(&test_file);
        assert_eq!(settings.line_ending, Some(LineEnding::Lf));
    }

    #[test]
    fn test_crlf_line_ending() {
        let temp = TempDir::new().unwrap();
        create_editorconfig(
            temp.path(),
            r#"
root = true

[*]
end_of_line = crlf
"#,
        );

        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, "").unwrap();

        EditorConfigSettings::clear_cache();
        let settings = EditorConfigSettings::for_file(&test_file);
        assert_eq!(settings.line_ending, Some(LineEnding::Crlf));
    }

    #[test]
    fn test_indent_style_tabs() {
        let temp = TempDir::new().unwrap();
        create_editorconfig(
            temp.path(),
            r#"
root = true

[*]
indent_style = tab
"#,
        );

        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, "").unwrap();

        EditorConfigSettings::clear_cache();
        let settings = EditorConfigSettings::for_file(&test_file);
        assert_eq!(settings.indent_style, Some(IndentStyle::Tabs));
    }

    #[test]
    fn test_indent_style_spaces() {
        let temp = TempDir::new().unwrap();
        create_editorconfig(
            temp.path(),
            r#"
root = true

[*]
indent_style = space
indent_size = 4
"#,
        );

        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, "").unwrap();

        EditorConfigSettings::clear_cache();
        let settings = EditorConfigSettings::for_file(&test_file);
        assert_eq!(settings.indent_style, Some(IndentStyle::Spaces));
        assert_eq!(settings.indent_size, Some(4));
    }

    #[test]
    fn test_charset_utf8() {
        let temp = TempDir::new().unwrap();
        create_editorconfig(
            temp.path(),
            r#"
root = true

[*]
charset = utf-8
"#,
        );

        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, "").unwrap();

        EditorConfigSettings::clear_cache();
        let settings = EditorConfigSettings::for_file(&test_file);
        assert_eq!(settings.charset, Some("utf-8".to_string()));
    }

    #[test]
    fn test_trim_trailing_whitespace() {
        let temp = TempDir::new().unwrap();
        create_editorconfig(
            temp.path(),
            r#"
root = true

[*]
trim_trailing_whitespace = true
"#,
        );

        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, "").unwrap();

        EditorConfigSettings::clear_cache();
        let settings = EditorConfigSettings::for_file(&test_file);
        assert_eq!(
            settings.trim_trailing_whitespace,
            Some(TrailingWhitespace::Remove)
        );
    }

    #[test]
    fn test_insert_final_newline() {
        let temp = TempDir::new().unwrap();
        create_editorconfig(
            temp.path(),
            r#"
root = true

[*]
insert_final_newline = true
"#,
        );

        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, "").unwrap();

        EditorConfigSettings::clear_cache();
        let settings = EditorConfigSettings::for_file(&test_file);
        assert_eq!(settings.insert_final_newline, Some(true));
    }

    #[test]
    fn test_no_editorconfig() {
        let temp = TempDir::new().unwrap();
        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, "").unwrap();

        EditorConfigSettings::clear_cache();
        let settings = EditorConfigSettings::for_file(&test_file);
        assert_eq!(settings.line_ending, None);
        assert_eq!(settings.indent_style, None);
    }

    #[test]
    fn test_to_indent_config() {
        let settings = EditorConfigSettings {
            indent_style: Some(IndentStyle::Spaces),
            indent_size: Some(4),
            ..Default::default()
        };
        let config = settings.to_indent_config().unwrap();
        assert_eq!(config.style, IndentStyle::Spaces);
        assert_eq!(config.width, 4);
    }

    #[test]
    fn test_pattern_matching() {
        let temp = TempDir::new().unwrap();
        create_editorconfig(
            temp.path(),
            r#"
root = true

[*.py]
indent_size = 4

[*.js]
indent_size = 2
"#,
        );

        let py_file = temp.path().join("test.py");
        let js_file = temp.path().join("test.js");
        fs::write(&py_file, "").unwrap();
        fs::write(&js_file, "").unwrap();

        EditorConfigSettings::clear_cache();

        let py_settings = EditorConfigSettings::for_file(&py_file);
        assert_eq!(py_settings.indent_size, Some(4));

        let js_settings = EditorConfigSettings::for_file(&js_file);
        assert_eq!(js_settings.indent_size, Some(2));
    }
}
