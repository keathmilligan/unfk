//! File discovery and filtering

use std::path::Path;

use anyhow::Result;
use ignore::WalkBuilder;

use crate::cli::Cli;
use crate::config::Config;

/// File discovery handles finding files to process
pub struct FileDiscovery<'a> {
    config: &'a Config,
    cli: &'a Cli,
}

impl<'a> FileDiscovery<'a> {
    pub fn new(config: &'a Config, cli: &'a Cli) -> Self {
        Self { config, cli }
    }

    /// Walk the given path and yield files to process
    pub fn walk(&self, path: &Path) -> Result<impl Iterator<Item = FileEntry>> {
        let mut builder = WalkBuilder::new(path);

        // Configure gitignore handling
        builder.git_ignore(!self.cli.no_gitignore);
        builder.git_global(!self.cli.no_gitignore);
        builder.git_exclude(!self.cli.no_gitignore);

        // Configure hidden file handling
        builder.hidden(!self.cli.no_hidden);

        // Add custom ignore patterns
        let ignore_globset = self.config.build_ignore_globset()?;

        // Build include filter
        let include_globset = self.config.build_include_globset(&self.cli.include)?;

        let max_size = self.config.max_file_size;
        let include_binary = self.cli.include_binary;

        Ok(builder
            .build()
            .filter_map(|entry| entry.ok())
            .filter(move |entry| {
                // Only process files
                entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)
            })
            .filter(move |entry| {
                // Check ignore patterns
                let path_str = entry.path().to_string_lossy();
                !ignore_globset.is_match(path_str.as_ref())
            })
            .filter(move |entry| {
                // Check include patterns
                if let Some(ref globset) = include_globset {
                    let path_str = entry.path().to_string_lossy();
                    globset.is_match(path_str.as_ref())
                } else {
                    true
                }
            })
            .filter(move |entry| {
                // Check file size
                if let Ok(metadata) = entry.metadata() {
                    metadata.len() as usize <= max_size
                } else {
                    true
                }
            })
            .filter_map(move |entry| {
                let path = entry.path();

                // Check for binary files
                if !include_binary && is_binary(path) {
                    return None;
                }

                Some(FileEntry {
                    path: path.to_path_buf(),
                })
            }))
    }
}

/// A discovered file entry
#[derive(Debug)]
pub struct FileEntry {
    path: std::path::PathBuf,
}

impl FileEntry {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// Check if a file appears to be binary
pub fn is_binary(path: &Path) -> bool {
    // Check extension first for known binary types
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let binary_extensions = [
            "png", "jpg", "jpeg", "gif", "bmp", "ico", "webp", "svg", "pdf", "zip", "tar", "gz",
            "bz2", "xz", "7z", "rar", "exe", "dll", "so", "dylib", "a", "o", "obj", "class", "jar",
            "war", "ear", "wasm", "pyc", "pyo", "beam", "db", "sqlite", "sqlite3", "mp3", "mp4",
            "avi", "mov", "mkv", "flac", "wav", "ogg", "woff", "woff2", "ttf", "otf", "eot",
        ];
        if binary_extensions.contains(&ext.to_lowercase().as_str()) {
            return true;
        }
    }

    // Check file content for binary signatures
    match std::fs::File::open(path) {
        Ok(mut file) => {
            use std::io::Read;
            let mut buffer = [0u8; 8192];
            match file.read(&mut buffer) {
                Ok(n) => {
                    // Check for NULL bytes (strong binary indicator)
                    if buffer[..n].contains(&0) {
                        return true;
                    }

                    // Check for binary file signatures
                    if n >= 4 {
                        // PNG
                        if buffer.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
                            return true;
                        }
                        // JPEG
                        if buffer.starts_with(&[0xFF, 0xD8, 0xFF]) {
                            return true;
                        }
                        // GIF
                        if buffer.starts_with(b"GIF8") {
                            return true;
                        }
                        // PDF
                        if buffer.starts_with(b"%PDF") {
                            return true;
                        }
                        // ZIP/JAR
                        if buffer.starts_with(&[0x50, 0x4B, 0x03, 0x04]) {
                            return true;
                        }
                        // ELF
                        if buffer.starts_with(&[0x7F, 0x45, 0x4C, 0x46]) {
                            return true;
                        }
                        // Mach-O
                        if buffer.starts_with(&[0xCF, 0xFA, 0xED, 0xFE])
                            || buffer.starts_with(&[0xFE, 0xED, 0xFA, 0xCF])
                        {
                            return true;
                        }
                    }

                    false
                }
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_is_binary_by_extension() {
        assert!(is_binary(Path::new("image.png")));
        assert!(is_binary(Path::new("archive.zip")));
        assert!(!is_binary(Path::new("source.rs")));
        assert!(!is_binary(Path::new("readme.md")));
    }

    #[test]
    fn test_is_binary_by_content() {
        // Text file
        let mut text_file = NamedTempFile::new().unwrap();
        writeln!(text_file, "Hello, world!").unwrap();
        assert!(!is_binary(text_file.path()));

        // Binary file (contains NULL)
        let mut binary_file = NamedTempFile::new().unwrap();
        binary_file.write_all(&[0x00, 0x01, 0x02]).unwrap();
        assert!(is_binary(binary_file.path()));
    }
}
