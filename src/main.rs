use std::io::{self, Write};
use std::process::ExitCode;

use anyhow::Result;
use colored::Colorize;
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
        println!("A fast CLI tool for scanning and repairing file formatting issues");
        return Ok(unfk::ExitCode::Success);
    }

    // Load configuration
    let config = Config::load(&cli)?;

    // Create reporter for output
    let reporter = Reporter::new(&cli);

    // Execute the appropriate command
    match &cli.command {
        Some(Commands::Scan { paths }) => {
            let result = run_scan(&cli, &config, &reporter, paths, false)?;
            Ok(result.exit_code)
        }
        None => {
            // Default behavior: run scan and potentially prompt for fix
            let result = run_scan(&cli, &config, &reporter, &cli.paths, false)?;
            // Only prompt if interactive (stdin is a TTY) and issues were found and not quiet
            let is_interactive = atty::is(atty::Stream::Stdin);
            if result.issues_found && is_interactive && !cli.quiet {
                // Prompt user to fix issues
                match prompt_fix()? {
                    true => {
                        // User wants to fix, run fix command
                        println!();
                        run_fix(&cli, &config, &reporter, &cli.paths, cli.dry_run, false)
                    }
                    false => {
                        // User declined, exit with success (not an error for default behavior)
                        Ok(unfk::ExitCode::Success)
                    }
                }
            } else {
                // Non-interactive or quiet mode: return scan result like explicit scan command
                Ok(result.exit_code)
            }
        }
        Some(Commands::Fix {
            paths,
            dry_run,
            all,
        }) => run_fix(
            &cli,
            &config,
            &reporter,
            paths,
            *dry_run || cli.dry_run,
            *all,
        ),
        Some(Commands::Init { force }) => run_init(*force),
        Some(Commands::Types { show }) => run_types(show.as_deref()),
        Some(Commands::Config { dump }) => run_config(&config, *dump),
    }
}

/// Result from running a scan, including exit code and whether issues were found
struct ScanResult {
    exit_code: unfk::ExitCode,
    issues_found: bool,
}

/// Prompt the user whether they want to fix discovered issues
fn prompt_fix() -> Result<bool> {
    print!(
        "\n{} Would you like to fix these issues? [Y/n] ",
        "?".cyan().bold()
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();
    // Default to "yes" if user just presses Enter
    Ok(input.is_empty() || input == "y" || input == "yes")
}

fn run_scan(
    cli: &Cli,
    config: &Config,
    reporter: &Reporter,
    paths: &[std::path::PathBuf],
    _is_explicit_scan: bool,
) -> Result<ScanResult> {
    let paths = if paths.is_empty() {
        vec![std::path::PathBuf::from(".")]
    } else {
        paths.to_vec()
    };

    let discovery = FileDiscovery::new(config, cli);
    let analyzer = unfk::analysis::Analyzer::new(config);

    let mut error_count = 0;
    let mut warning_count = 0;
    let mut files_with_issues = 0;

    for path in &paths {
        for entry in discovery.walk(path)? {
            let file_path = entry.path();

            match analyzer.analyze(file_path) {
                Ok(issues) if !issues.is_empty() => {
                    files_with_issues += 1;
                    for issue in &issues {
                        if issue.is_fixable() {
                            error_count += 1;
                        } else {
                            warning_count += 1;
                        }
                    }
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

    reporter.report_summary(files_with_issues, error_count, warning_count);

    let total_issues = error_count + warning_count;
    let exit_code = if total_issues > 0 {
        unfk::ExitCode::IssuesFound
    } else {
        unfk::ExitCode::Success
    };

    Ok(ScanResult {
        exit_code,
        issues_found: total_issues > 0,
    })
}

fn run_fix(
    cli: &Cli,
    config: &Config,
    reporter: &Reporter,
    paths: &[std::path::PathBuf],
    dry_run: bool,
    fix_all: bool,
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
                    // Filter issues: only fixable (errors) unless --all is specified
                    let issues_to_fix: Vec<_> = if fix_all {
                        issues.clone()
                    } else {
                        issues.iter().filter(|i| i.is_fixable()).cloned().collect()
                    };

                    if issues_to_fix.is_empty() {
                        continue;
                    }

                    if dry_run {
                        reporter.report_would_fix(file_path, &issues_to_fix);
                        total_fixed += 1;
                    } else {
                        match repairer.repair(file_path, &issues_to_fix) {
                            Ok(()) => {
                                reporter.report_fixed(file_path, &issues_to_fix);
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
