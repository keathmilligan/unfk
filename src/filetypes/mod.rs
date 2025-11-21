//! File type detection and default settings

use std::path::Path;

use crate::config::{IndentConfig, IndentStyle, LineEnding};

/// Registry of known file types and their default settings
pub struct FileTypeRegistry {
    types: Vec<FileType>,
}

impl FileTypeRegistry {
    pub fn new() -> Self {
        Self {
            types: vec![
                // Makefiles - tabs required
                FileType {
                    name: "makefile".to_string(),
                    extensions: vec!["Makefile", "makefile", "GNUmakefile", "mk"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Tabs,
                        width: 4,
                    },
                    tabs_required: true,
                },
                // Go - tabs by convention
                FileType {
                    name: "go".to_string(),
                    extensions: vec!["go"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Tabs,
                        width: 4,
                    },
                    tabs_required: false,
                },
                // Rust - 4 spaces
                FileType {
                    name: "rust".to_string(),
                    extensions: vec!["rs"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                },
                // Python - 4 spaces
                FileType {
                    name: "python".to_string(),
                    extensions: vec!["py", "pyi", "pyw"],
                    shebangs: vec!["python", "python3"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                },
                // JavaScript/TypeScript - 2 spaces
                FileType {
                    name: "javascript".to_string(),
                    extensions: vec!["js", "mjs", "cjs", "jsx"],
                    shebangs: vec!["node", "nodejs"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                },
                FileType {
                    name: "typescript".to_string(),
                    extensions: vec!["ts", "mts", "cts", "tsx"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                },
                // YAML - 2 spaces
                FileType {
                    name: "yaml".to_string(),
                    extensions: vec!["yml", "yaml"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                },
                // JSON - 2 spaces
                FileType {
                    name: "json".to_string(),
                    extensions: vec!["json", "jsonc"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                },
                // TOML - 2 spaces
                FileType {
                    name: "toml".to_string(),
                    extensions: vec!["toml"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                },
                // Markdown
                FileType {
                    name: "markdown".to_string(),
                    extensions: vec!["md", "markdown", "mdown", "mkd"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                },
                // Shell scripts
                FileType {
                    name: "shell".to_string(),
                    extensions: vec!["sh", "bash", "zsh", "fish"],
                    shebangs: vec!["sh", "bash", "zsh", "fish"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                },
                // Windows batch - CRLF
                FileType {
                    name: "batch".to_string(),
                    extensions: vec!["bat", "cmd"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Crlf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                },
                // PowerShell - CRLF
                FileType {
                    name: "powershell".to_string(),
                    extensions: vec!["ps1", "psm1", "psd1"],
                    shebangs: vec!["pwsh", "powershell"],
                    default_line_ending: LineEnding::Crlf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                },
                // C/C++
                FileType {
                    name: "c".to_string(),
                    extensions: vec!["c", "h"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                },
                FileType {
                    name: "cpp".to_string(),
                    extensions: vec!["cpp", "cc", "cxx", "hpp", "hh", "hxx"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                },
                // Java
                FileType {
                    name: "java".to_string(),
                    extensions: vec!["java"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                },
                // Ruby
                FileType {
                    name: "ruby".to_string(),
                    extensions: vec!["rb", "rake", "gemspec"],
                    shebangs: vec!["ruby"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                },
                // XML/HTML
                FileType {
                    name: "xml".to_string(),
                    extensions: vec!["xml", "xsd", "xsl", "xslt", "svg"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                },
                FileType {
                    name: "html".to_string(),
                    extensions: vec!["html", "htm", "xhtml"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                },
                // CSS
                FileType {
                    name: "css".to_string(),
                    extensions: vec!["css", "scss", "sass", "less"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                },
                // SQL
                FileType {
                    name: "sql".to_string(),
                    extensions: vec!["sql"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                },
                // Plain text
                FileType {
                    name: "text".to_string(),
                    extensions: vec!["txt", "text"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                },
            ],
        }
    }

    /// Get all file types
    pub fn all(&self) -> &[FileType] {
        &self.types
    }

    /// Get a file type by name
    pub fn get_by_name(&self, name: &str) -> Option<&FileType> {
        self.types
            .iter()
            .find(|ft| ft.name.eq_ignore_ascii_case(name))
    }

    /// Detect file type from path
    pub fn detect(&self, path: &Path) -> Option<&FileType> {
        // Check filename first (for Makefile, etc.)
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            for ft in &self.types {
                if ft.extensions.iter().any(|e| e.eq_ignore_ascii_case(filename)) {
                    return Some(ft);
                }
            }
        }

        // Check extension
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            for ft in &self.types {
                if ft.extensions.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                    return Some(ft);
                }
            }
        }

        // Try shebang detection
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Some(first_line) = content.lines().next() {
                if first_line.starts_with("#!") {
                    for ft in &self.types {
                        for shebang in &ft.shebangs {
                            if first_line.contains(shebang) {
                                return Some(ft);
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

impl Default for FileTypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// A known file type with its default settings
#[derive(Debug, Clone)]
pub struct FileType {
    /// Name of the file type
    pub name: String,
    /// File extensions (without dot) or filenames
    pub extensions: Vec<&'static str>,
    /// Shebang identifiers
    pub shebangs: Vec<&'static str>,
    /// Default line ending
    pub default_line_ending: LineEnding,
    /// Default indentation
    pub default_indent: IndentConfig,
    /// Whether tabs are strictly required (like Makefiles)
    pub tabs_required: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_by_extension() {
        let registry = FileTypeRegistry::new();

        let rust_type = registry.detect(Path::new("main.rs"));
        assert!(rust_type.is_some());
        assert_eq!(rust_type.unwrap().name, "rust");

        let py_type = registry.detect(Path::new("script.py"));
        assert!(py_type.is_some());
        assert_eq!(py_type.unwrap().name, "python");
    }

    #[test]
    fn test_detect_makefile() {
        let registry = FileTypeRegistry::new();

        let makefile = registry.detect(Path::new("Makefile"));
        assert!(makefile.is_some());
        assert_eq!(makefile.unwrap().name, "makefile");
        assert!(makefile.unwrap().tabs_required);
    }

    #[test]
    fn test_get_by_name() {
        let registry = FileTypeRegistry::new();

        let rust_type = registry.get_by_name("rust");
        assert!(rust_type.is_some());
        assert_eq!(rust_type.unwrap().name, "rust");

        let unknown = registry.get_by_name("unknown");
        assert!(unknown.is_none());
    }
}
