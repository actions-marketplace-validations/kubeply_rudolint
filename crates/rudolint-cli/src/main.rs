mod cli;

use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::Parser;
use ignore::WalkBuilder;

use crate::cli::{Cli, Command, OutputFormat};
use rudolint_config::Config;
use rudolint_diagnostics::Finding;
use rudolint_dockerfile::parse_dockerfile;
use rudolint_rules::{RuleEngine, RuleStatus};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command.unwrap_or_default() {
        Command::Check(args) => run_check(args),
        Command::Rules(args) => run_rules(args),
    }
}

fn run_check(args: cli::CheckArgs) -> Result<()> {
    let config = Config::load(args.config.as_deref())?;
    let engine = RuleEngine::new(args.profile, config);
    let inputs = resolve_inputs(&args.paths)?;
    let mut findings = Vec::new();

    if inputs.is_empty() {
        let mut source = String::new();
        io::stdin()
            .read_to_string(&mut source)
            .context("failed to read Dockerfile from stdin")?;
        findings.extend(lint_source(Path::new("<stdin>"), &source, &engine)?);
    } else {
        for path in inputs {
            let source = fs::read_to_string(&path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            findings.extend(lint_source(&path, &source, &engine)?);
        }
    }

    let rendered = match args.format {
        OutputFormat::Human => rudolint_output::human(&findings),
        OutputFormat::Json => rudolint_output::json(&findings)?,
        OutputFormat::Sarif => rudolint_output::sarif(&findings)?,
    };
    print!("{rendered}");

    if findings
        .iter()
        .any(|finding| finding.severity.is_failure(args.failure_threshold))
    {
        bail!("lint findings met or exceeded the failure threshold");
    }

    Ok(())
}

fn run_rules(args: cli::RulesArgs) -> Result<()> {
    let engine = RuleEngine::new(args.profile, Config::default());
    for rule in engine.catalog() {
        if args.implemented && rule.status != RuleStatus::Implemented {
            continue;
        }
        println!(
            "{:<8} {:<8} {:<12} {}",
            rule.code, rule.severity, rule.status, rule.summary
        );
    }
    Ok(())
}

fn lint_source(path: &Path, source: &str, engine: &RuleEngine) -> Result<Vec<Finding>> {
    let document = parse_dockerfile(source).with_context(|| {
        format!(
            "failed to parse {}",
            path.to_str().unwrap_or("<non-utf8-path>")
        )
    })?;
    Ok(engine
        .lint(&document)
        .into_iter()
        .map(|finding| finding.with_path(path))
        .collect())
}

fn resolve_inputs(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for path in paths {
        if path.is_file() {
            files.push(path.clone());
            continue;
        }
        if !path.is_dir() {
            bail!("{} is not a file or directory", path.display());
        }
        for entry in WalkBuilder::new(path).hidden(false).build() {
            let entry = entry?;
            if !entry.file_type().is_some_and(|kind| kind.is_file()) {
                continue;
            }
            let name = entry.file_name().to_string_lossy();
            if name == "Dockerfile" || name.starts_with("Dockerfile.") {
                files.push(entry.into_path());
            }
        }
    }
    files.sort();
    Ok(files)
}
