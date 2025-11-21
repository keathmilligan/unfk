use std::process::ExitCode;

use anyhow::Result;
use unfk::cli::{Cli, Commands, BANNER};
use unfk::config::Config;
use unfk::discovery::FileDiscovery;
use unfk::output::Reporter;

fn main() -> ExitCode {
    match run() {
        Ok(code) => code.into(),
        Err(e) => {
            eprintln!("error: {e:#}");
            unfk::ExitCode::ConfigError.into()
        }
    }
}

fn run() -> Result<unfk::ExitCode> {
    let cli = Cli::parse_args();

    // Handle version flag with banner
    if cli.version {
        print!("{}", BANNER);
        println!("unfk {}", env!("CARGO_PKG_VERSION"));
        println!("A fast, modern CLI tool for scanning and repairing file formatting issues");
        return Ok(unfk::ExitCode::Success);
    }

    // Load configuration
    let config = Config::load(&cli)?;

    // Create reporter for output
    let reporter = Reporter::new(&cli);

    // Execute the appropriate command
    match &cli.command {
        Some(Commands::Scan { paths }) => {
            run_scan(&cli, &config, &reporter, paths)
        }
        None => {
            run_scan(&cli, &config, &reporter, &cli.paths)
        }
        Some(Commands::Fix { paths, dry_run }) => {
            run_fix(&cli, &config, &reporter, paths, *dry_run || cli.dry_run)
        }
        Some(Commands::Init { force }) => run_init(*force),
        Some(Commands::Types { show }) => run_types(show.as_deref()),
        Some(Commands::Config { dump }) => run_config(&config, *dump),
    }
}

fn run_scan(
    cli: &Cli,
    config: &Config,
    reporter: &Reporter,
    paths: &[std::path::PathBuf],
) -> Result<unfk::ExitCode> {
    let paths = if paths.is_empty() {
        vec![std::path::PathBuf::from(".")]
    } else {
        paths.to_vec()
    };

    let discovery = FileDiscovery::new(config, cli);
    let analyzer = unfk::analysis::Analyzer::new(config);

    let mut total_issues = 0;
    let mut files_with_issues = 0;

    for path in &paths {
        for entry in discovery.walk(path)? {
            let file_path = entry.path();

            match analyzer.analyze(file_path) {
                Ok(issues) if !issues.is_empty() => {
                    files_with_issues += 1;
                    total_issues += issues.len();
                    reporter.report_file_issues(file_path, &issues);
                }
                Ok(_) => {
                    reporter.report_file_ok(file_path);
                }
                Err(e) => {
                    reporter.report_error(file_path, &e);
                }
            }
        }
    }

    reporter.report_summary(files_with_issues, total_issues);

    if total_issues > 0 {
        Ok(unfk::ExitCode::IssuesFound)
    } else {
        Ok(unfk::ExitCode::Success)
    }
}

fn run_fix(
    cli: &Cli,
    config: &Config,
    reporter: &Reporter,
    paths: &[std::path::PathBuf],
    dry_run: bool,
) -> Result<unfk::ExitCode> {
    let paths = if paths.is_empty() {
        vec![std::path::PathBuf::from(".")]
    } else {
        paths.to_vec()
    };

    let discovery = FileDiscovery::new(config, cli);
    let analyzer = unfk::analysis::Analyzer::new(config);
    let repairer = unfk::repair::Repairer::new(config);

    let mut total_fixed = 0;
    let mut total_failed = 0;

    for path in &paths {
        for entry in discovery.walk(path)? {
            let file_path = entry.path();

            match analyzer.analyze(file_path) {
                Ok(issues) if !issues.is_empty() => {
                    if dry_run {
                        reporter.report_would_fix(file_path, &issues);
                        total_fixed += 1;
                    } else {
                        match repairer.repair(file_path, &issues) {
                            Ok(()) => {
                                reporter.report_fixed(file_path, &issues);
                                total_fixed += 1;
                            }
                            Err(e) => {
                                reporter.report_error(file_path, &e);
                                total_failed += 1;
                            }
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    reporter.report_error(file_path, &e);
                    total_failed += 1;
                }
            }
        }
    }

    reporter.report_fix_summary(total_fixed, total_failed, dry_run);

    if total_failed > 0 {
        Ok(unfk::ExitCode::IssuesFound)
    } else {
        Ok(unfk::ExitCode::Success)
    }
}

fn run_init(force: bool) -> Result<unfk::ExitCode> {
    let config_path = std::path::Path::new(".unfkrc.toml");

    if config_path.exists() && !force {
        eprintln!(
            "Configuration file already exists: {}",
            config_path.display()
        );
        eprintln!("Use --force to overwrite");
        return Ok(unfk::ExitCode::ConfigError);
    }

    let default_config = include_str!("config/default.toml");
    std::fs::write(config_path, default_config)?;
    println!("Created {}", config_path.display());

    Ok(unfk::ExitCode::Success)
}

fn run_types(show: Option<&str>) -> Result<unfk::ExitCode> {
    let registry = unfk::filetypes::FileTypeRegistry::new();

    if let Some(type_name) = show {
        if let Some(file_type) = registry.get_by_name(type_name) {
            println!("File type: {}", file_type.name);
            println!("  Extensions: {}", file_type.extensions.join(", "));
            println!("  Line ending: {:?}", file_type.default_line_ending);
            println!("  Indentation: {:?}", file_type.default_indent);
        } else {
            eprintln!("Unknown file type: {type_name}");
            return Ok(unfk::ExitCode::ConfigError);
        }
    } else {
        println!("Known file types:\n");
        let mut file_types: Vec<_> = registry.all().iter().collect();
        file_types.sort_by(|a, b| a.name.cmp(&b.name));
        for file_type in file_types {
            println!(
                "  {:<15} {}",
                file_type.name,
                file_type.extensions.join(", ")
            );
        }
    }

    Ok(unfk::ExitCode::Success)
}

fn run_config(config: &Config, dump: bool) -> Result<unfk::ExitCode> {
    if dump {
        println!("{}", toml::to_string_pretty(config)?);
    } else {
        if let Some(path) = &config.source_path {
            println!("Configuration loaded from: {}", path.display());
        } else {
            println!("Using default configuration (no config file found)");
        }
    }

    Ok(unfk::ExitCode::Success)
}
