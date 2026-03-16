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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
                },
                // Python - 4 spaces (PEP8 strongly discourages tabs)
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
                    tabs_forbidden: true,
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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
                },
                // YAML - 2 spaces (tabs forbidden per YAML spec)
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
                    tabs_forbidden: true,
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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
                },
                // C# - 4 spaces
                FileType {
                    name: "csharp".to_string(),
                    extensions: vec!["cs", "csx"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Swift - 4 spaces
                FileType {
                    name: "swift".to_string(),
                    extensions: vec!["swift"],
                    shebangs: vec!["swift"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Haskell - 2 spaces
                FileType {
                    name: "haskell".to_string(),
                    extensions: vec!["hs", "lhs"],
                    shebangs: vec!["runhaskell", "runghc"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Assembly
                FileType {
                    name: "assembly".to_string(),
                    extensions: vec!["asm", "s", "S", "nasm", "masm"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Kotlin - 4 spaces
                FileType {
                    name: "kotlin".to_string(),
                    extensions: vec!["kt", "kts"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Scala - 2 spaces
                FileType {
                    name: "scala".to_string(),
                    extensions: vec!["scala", "sc"],
                    shebangs: vec!["scala"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // PHP - 4 spaces
                FileType {
                    name: "php".to_string(),
                    extensions: vec!["php", "phtml", "php3", "php4", "php5", "php7", "phps"],
                    shebangs: vec!["php"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Perl - 4 spaces
                FileType {
                    name: "perl".to_string(),
                    extensions: vec!["pl", "pm", "t", "pod"],
                    shebangs: vec!["perl"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Lua - 2 spaces
                FileType {
                    name: "lua".to_string(),
                    extensions: vec!["lua"],
                    shebangs: vec!["lua"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // R - 2 spaces
                FileType {
                    name: "r".to_string(),
                    extensions: vec!["r", "R", "rmd", "Rmd"],
                    shebangs: vec!["Rscript"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Elixir - 2 spaces
                FileType {
                    name: "elixir".to_string(),
                    extensions: vec!["ex", "exs"],
                    shebangs: vec!["elixir"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Erlang - 4 spaces
                FileType {
                    name: "erlang".to_string(),
                    extensions: vec!["erl", "hrl"],
                    shebangs: vec!["escript"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Clojure - 2 spaces
                FileType {
                    name: "clojure".to_string(),
                    extensions: vec!["clj", "cljs", "cljc", "edn"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // F# - 4 spaces
                FileType {
                    name: "fsharp".to_string(),
                    extensions: vec!["fs", "fsi", "fsx", "fsscript"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Objective-C - 4 spaces
                FileType {
                    name: "objc".to_string(),
                    extensions: vec!["m", "mm"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Groovy - 4 spaces
                FileType {
                    name: "groovy".to_string(),
                    extensions: vec!["groovy", "gvy", "gy", "gsh"],
                    shebangs: vec!["groovy"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Dart - 2 spaces
                FileType {
                    name: "dart".to_string(),
                    extensions: vec!["dart"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Julia - 4 spaces
                FileType {
                    name: "julia".to_string(),
                    extensions: vec!["jl"],
                    shebangs: vec!["julia"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Zig - 4 spaces
                FileType {
                    name: "zig".to_string(),
                    extensions: vec!["zig"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Nim - 2 spaces
                FileType {
                    name: "nim".to_string(),
                    extensions: vec!["nim", "nims"],
                    shebangs: vec!["nim"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Crystal - 2 spaces
                FileType {
                    name: "crystal".to_string(),
                    extensions: vec!["cr"],
                    shebangs: vec!["crystal"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // V - tabs
                FileType {
                    name: "vlang".to_string(),
                    extensions: vec!["v"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Tabs,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // OCaml - 2 spaces
                FileType {
                    name: "ocaml".to_string(),
                    extensions: vec!["ml", "mli"],
                    shebangs: vec!["ocaml"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // ReasonML - 2 spaces
                FileType {
                    name: "reason".to_string(),
                    extensions: vec!["re", "rei"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Solidity - 4 spaces
                FileType {
                    name: "solidity".to_string(),
                    extensions: vec!["sol"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Terraform/HCL - 2 spaces
                FileType {
                    name: "hcl".to_string(),
                    extensions: vec!["tf", "tfvars", "hcl"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Protocol Buffers - 2 spaces
                FileType {
                    name: "protobuf".to_string(),
                    extensions: vec!["proto"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // GraphQL - 2 spaces
                FileType {
                    name: "graphql".to_string(),
                    extensions: vec!["graphql", "gql"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Dockerfile
                FileType {
                    name: "dockerfile".to_string(),
                    extensions: vec!["Dockerfile", "dockerfile", "Containerfile"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Nix - 2 spaces
                FileType {
                    name: "nix".to_string(),
                    extensions: vec!["nix"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Puppet - 2 spaces
                FileType {
                    name: "puppet".to_string(),
                    extensions: vec!["pp"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Ansible/Jinja templates
                FileType {
                    name: "jinja".to_string(),
                    extensions: vec!["j2", "jinja", "jinja2"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // JSONL/JSON Lines
                FileType {
                    name: "jsonl".to_string(),
                    extensions: vec!["jsonl", "ndjson"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // CSV/TSV
                FileType {
                    name: "csv".to_string(),
                    extensions: vec!["csv", "tsv"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // INI/Config files
                FileType {
                    name: "ini".to_string(),
                    extensions: vec!["ini", "cfg", "conf", "properties"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Diff/Patch
                FileType {
                    name: "diff".to_string(),
                    extensions: vec!["diff", "patch"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // reStructuredText
                FileType {
                    name: "rst".to_string(),
                    extensions: vec!["rst", "rest"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 3,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // AsciiDoc
                FileType {
                    name: "asciidoc".to_string(),
                    extensions: vec!["adoc", "asciidoc", "asc"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // LaTeX
                FileType {
                    name: "latex".to_string(),
                    extensions: vec!["tex", "sty", "cls", "bib"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // CMake
                FileType {
                    name: "cmake".to_string(),
                    extensions: vec!["cmake", "CMakeLists.txt"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Windows Registry files - CRLF required
                FileType {
                    name: "registry".to_string(),
                    extensions: vec!["reg"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Crlf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Windows INF files - CRLF
                FileType {
                    name: "inf".to_string(),
                    extensions: vec!["inf"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Crlf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 2,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // Visual Basic - CRLF
                FileType {
                    name: "vb".to_string(),
                    extensions: vec!["vb", "vbs", "vba", "bas", "cls", "frm"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Crlf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // macOS Property List (XML-based)
                FileType {
                    name: "plist".to_string(),
                    extensions: vec!["plist"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Tabs,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // macOS/iOS Storyboard and XIB files
                FileType {
                    name: "xib".to_string(),
                    extensions: vec!["xib", "storyboard"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Spaces,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // macOS Entitlements
                FileType {
                    name: "entitlements".to_string(),
                    extensions: vec!["entitlements"],
                    shebangs: vec![],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Tabs,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
                },
                // AppleScript
                FileType {
                    name: "applescript".to_string(),
                    extensions: vec!["applescript", "scpt"],
                    shebangs: vec!["osascript"],
                    default_line_ending: LineEnding::Lf,
                    default_indent: IndentConfig {
                        style: IndentStyle::Tabs,
                        width: 4,
                    },
                    tabs_required: false,
                    tabs_forbidden: false,
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
                    tabs_forbidden: false,
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
                if ft
                    .extensions
                    .iter()
                    .any(|e| e.eq_ignore_ascii_case(filename))
                {
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
    /// Whether tabs are forbidden (like YAML, Python)
    pub tabs_forbidden: bool,
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

    #[test]
    fn test_detect_csharp() {
        let registry = FileTypeRegistry::new();

        let cs_type = registry.detect(Path::new("Program.cs"));
        assert!(cs_type.is_some());
        assert_eq!(cs_type.unwrap().name, "csharp");

        let csx_type = registry.detect(Path::new("script.csx"));
        assert!(csx_type.is_some());
        assert_eq!(csx_type.unwrap().name, "csharp");
    }

    #[test]
    fn test_detect_swift() {
        let registry = FileTypeRegistry::new();

        let swift_type = registry.detect(Path::new("main.swift"));
        assert!(swift_type.is_some());
        assert_eq!(swift_type.unwrap().name, "swift");
    }

    #[test]
    fn test_detect_haskell() {
        let registry = FileTypeRegistry::new();

        let hs_type = registry.detect(Path::new("Main.hs"));
        assert!(hs_type.is_some());
        assert_eq!(hs_type.unwrap().name, "haskell");

        let lhs_type = registry.detect(Path::new("Literate.lhs"));
        assert!(lhs_type.is_some());
        assert_eq!(lhs_type.unwrap().name, "haskell");
    }

    #[test]
    fn test_detect_assembly() {
        let registry = FileTypeRegistry::new();

        let asm_type = registry.detect(Path::new("boot.asm"));
        assert!(asm_type.is_some());
        assert_eq!(asm_type.unwrap().name, "assembly");
    }

    #[test]
    fn test_detect_kotlin() {
        let registry = FileTypeRegistry::new();

        let kt_type = registry.detect(Path::new("Main.kt"));
        assert!(kt_type.is_some());
        assert_eq!(kt_type.unwrap().name, "kotlin");

        let kts_type = registry.detect(Path::new("build.gradle.kts"));
        assert!(kts_type.is_some());
        assert_eq!(kts_type.unwrap().name, "kotlin");
    }

    #[test]
    fn test_detect_jsonl() {
        let registry = FileTypeRegistry::new();

        let jsonl_type = registry.detect(Path::new("data.jsonl"));
        assert!(jsonl_type.is_some());
        assert_eq!(jsonl_type.unwrap().name, "jsonl");

        let ndjson_type = registry.detect(Path::new("events.ndjson"));
        assert!(ndjson_type.is_some());
        assert_eq!(ndjson_type.unwrap().name, "jsonl");
    }

    #[test]
    fn test_detect_csv() {
        let registry = FileTypeRegistry::new();

        let csv_type = registry.detect(Path::new("data.csv"));
        assert!(csv_type.is_some());
        assert_eq!(csv_type.unwrap().name, "csv");

        let tsv_type = registry.detect(Path::new("data.tsv"));
        assert!(tsv_type.is_some());
        assert_eq!(tsv_type.unwrap().name, "csv");
    }

    #[test]
    fn test_detect_additional_languages() {
        let registry = FileTypeRegistry::new();

        // Scala
        let scala_type = registry.detect(Path::new("Main.scala"));
        assert!(scala_type.is_some());
        assert_eq!(scala_type.unwrap().name, "scala");

        // PHP
        let php_type = registry.detect(Path::new("index.php"));
        assert!(php_type.is_some());
        assert_eq!(php_type.unwrap().name, "php");

        // Perl
        let perl_type = registry.detect(Path::new("script.pl"));
        assert!(perl_type.is_some());
        assert_eq!(perl_type.unwrap().name, "perl");

        // Lua
        let lua_type = registry.detect(Path::new("init.lua"));
        assert!(lua_type.is_some());
        assert_eq!(lua_type.unwrap().name, "lua");

        // Elixir
        let elixir_type = registry.detect(Path::new("app.ex"));
        assert!(elixir_type.is_some());
        assert_eq!(elixir_type.unwrap().name, "elixir");

        // Dart
        let dart_type = registry.detect(Path::new("main.dart"));
        assert!(dart_type.is_some());
        assert_eq!(dart_type.unwrap().name, "dart");

        // Julia
        let julia_type = registry.detect(Path::new("script.jl"));
        assert!(julia_type.is_some());
        assert_eq!(julia_type.unwrap().name, "julia");

        // Zig
        let zig_type = registry.detect(Path::new("main.zig"));
        assert!(zig_type.is_some());
        assert_eq!(zig_type.unwrap().name, "zig");

        // Terraform
        let tf_type = registry.detect(Path::new("main.tf"));
        assert!(tf_type.is_some());
        assert_eq!(tf_type.unwrap().name, "hcl");

        // GraphQL
        let gql_type = registry.detect(Path::new("schema.graphql"));
        assert!(gql_type.is_some());
        assert_eq!(gql_type.unwrap().name, "graphql");
    }
}
