//! Runs one of the `unix` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

#[cfg(target_family = "unix")]
mod nix;
#[cfg(target_family = "unix")]
mod rustix;

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
    /// Runs the `nix` example.
    #[cfg(target_family = "unix")]
    #[command(name = "nix")]
    Nix,
    /// Runs the `rustix` example.
    #[cfg(target_family = "unix")]
    #[command(name = "rustix")]
    Rustix,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(target_family = "unix")]
            Commands::Nix => {
                nix::run();
            }
            #[cfg(target_family = "unix")]
            Commands::Rustix => {
                rustix::run();
            }
        }
    }
}
