//! Command-line interface definition using clap

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

/// ASCII banner for help and version output
pub const BANNER: &str = r#"
██╗   ██╗███╗   ██╗███████╗██╗  ██╗
██║   ██║████╗  ██║██╔════╝██║ ██╔╝
██║   ██║██╔██╗ ██║█████╗  █████╔╝
██║   ██║██║╚██╗██║██╔══╝  ██╔═██╗
╚██████╔╝██║ ╚████║██║     ██║  ██╗
 ╚═════╝ ╚═╝  ╚═══╝╚═╝     ╚═╝  ╚═╝
"#;

/// A fast, modern CLI tool for scanning and repairing file formatting issues
#[derive(Parser, Debug)]
#[command(name = "unfk", version, about, long_about = None)]
#[command(before_help = BANNER)]
#[command(propagate_version = true, disable_version_flag = true)]
pub struct Cli {
    /// Paths to scan or fix (defaults to current directory)
    #[arg(global = true)]
    pub paths: Vec<PathBuf>,

    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Show verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress all output (exit code only)
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Path to configuration file
    #[arg(short, long, global = true, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Don't respect .gitignore files
    #[arg(long, global = true)]
    pub no_gitignore: bool,

    /// Don't read .editorconfig files
    #[arg(long, global = true)]
    pub no_editorconfig: bool,

    /// Don't skip hidden files
    #[arg(long, global = true)]
    pub no_hidden: bool,

    /// Include binary files
    #[arg(long, global = true)]
    pub include_binary: bool,

    /// Maximum file size to process (e.g., "10MB")
    #[arg(long, global = true, value_name = "SIZE")]
    pub max_size: Option<String>,

    /// Only include files matching these patterns
    #[arg(long, global = true, value_name = "PATTERN")]
    pub include: Vec<String>,

    /// Exclude files matching these patterns
    #[arg(long, global = true, value_name = "PATTERN")]
    pub exclude: Vec<String>,

    /// Line ending style to enforce
    #[arg(long, global = true, value_name = "STYLE")]
    pub line_ending: Option<LineEndingArg>,

    /// Indentation style to enforce
    #[arg(long, global = true, value_name = "STYLE")]
    pub indent: Option<String>,

    /// Target encoding
    #[arg(long, global = true, value_name = "ENCODING")]
    pub encoding: Option<String>,

    /// Show what would be done without making changes
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Print version information
    #[arg(short = 'V', long)]
    pub version: bool,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Scan files for formatting issues (default)
    Scan {
        /// Paths to scan
        #[arg()]
        paths: Vec<PathBuf>,
    },

    /// Fix formatting issues in files
    Fix {
        /// Paths to fix
        #[arg()]
        paths: Vec<PathBuf>,

        /// Show what would be done without making changes
        #[arg(long)]
        dry_run: bool,

        /// Fix all issues including warnings (e.g., trailing whitespace, missing final newline)
        #[arg(long, short = 'a')]
        all: bool,
    },

    /// Create a default configuration file
    Init {
        /// Overwrite existing configuration
        #[arg(long)]
        force: bool,
    },

    /// List known file types and their defaults
    Types {
        /// Show details for a specific type
        #[arg(long, value_name = "TYPE")]
        show: Option<String>,
    },

    /// Show or dump configuration
    Config {
        /// Dump effective configuration as TOML
        #[arg(long)]
        dump: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum LineEndingArg {
    /// Unix-style line endings (LF)
    Lf,
    /// Windows-style line endings (CRLF)
    Crlf,
    /// Auto-detect from file content
    Auto,
}
