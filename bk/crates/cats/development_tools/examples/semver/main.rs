//! Semver tools examples.

use clap::Parser;
use clap::Subcommand;

#[cfg(not(windows))]
mod semver_command;
mod semver_complex;
mod semver_latest;
mod semver_parse;
mod semver_prerelease;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(not(windows))]
    #[command(name = "semver_command")]
    Command,
    #[command(name = "semver_complex")]
    Complex,
    #[command(name = "semver_latest")]
    Latest,
    #[command(name = "semver_parse")]
    Parse,
    #[command(name = "semver_prerelease")]
    Prerelease,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(not(windows))]
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
