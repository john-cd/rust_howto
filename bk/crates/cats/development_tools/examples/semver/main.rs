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
    SemverCommand,
    #[command(name = "semver_complex")]
    SemverComplex,
    #[command(name = "semver_latest")]
    SemverLatest,
    #[command(name = "semver_parse")]
    SemverParse,
    #[command(name = "semver_prerelease")]
    SemverPrerelease,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(not(windows))]
            Commands::SemverCommand => {
                semver_command::run();
            }
            Commands::SemverComplex => {
                semver_complex::run();
            }
            Commands::SemverLatest => {
                semver_latest::run();
            }
            Commands::SemverParse => {
                semver_parse::run();
            }
            Commands::SemverPrerelease => {
                semver_prerelease::run();
            }
        }
    }
}
