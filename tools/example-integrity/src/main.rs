//! Command-line entry point for the chapter and example integrity inventory.
//!
//! TODO Reviewed ??-??-2026.

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Context as _;
use anyhow::Result;
use clap::Parser;
use example_integrity::build_report;

/// Read-only chapter, include, and Cargo target inventory.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Repository root to scan. Defaults to the repository containing this tool.
    #[arg(long)]
    repo_root: Option<PathBuf>,
    /// Return a failing exit status when the report contains integrity errors.
    #[arg(long)]
    fail: bool,
}

/// Returns the repository root containing this package.
fn default_repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .map_or_else(|| PathBuf::from("."), std::path::Path::to_path_buf)
}

/// Scans the repository and writes a stable JSON report to stdout.
fn run() -> Result<ExitCode> {
    let args = Args::parse();
    let repo_root = args.repo_root.unwrap_or_else(default_repository_root);
    let report = build_report(&repo_root)
        .with_context(|| format!("could not scan {}", repo_root.display()))?;
    serde_json::to_writer_pretty(std::io::stdout().lock(), &report)
        .context("could not write the JSON report")?;
    println!();

    Ok(if args.fail && report.has_errors() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// Starts the command-line application and reports top-level errors.
fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
}
