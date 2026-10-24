//! Semver tools examples.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod semver_command;
mod semver_complex;
mod semver_latest;
mod semver_parse;
mod semver_prerelease;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the `semver_command` example.
    #[command(name = "semver_command")]
    Command,
    /// Runs the `semver_complex` example.
    #[command(name = "semver_complex")]
    Complex,
    /// Runs the `semver_latest` example.
    #[command(name = "semver_latest")]
    Latest,
    /// Runs the `semver_parse` example.
    #[command(name = "semver_parse")]
    Parse,
    /// Runs the `semver_prerelease` example.
    #[command(name = "semver_prerelease")]
    Prerelease,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Command => {
                semver_command::run()?;
            }
            Commands::Complex => {
                semver_complex::run()?;
            }
            Commands::Latest => {
                semver_latest::run()?;
            }
            Commands::Parse => {
                semver_parse::run()?;
            }
            Commands::Prerelease => {
                semver_prerelease::run()?;
            }
        }
    }

    Ok(())
}
