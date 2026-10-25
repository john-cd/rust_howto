//! Runs one of the `utilities` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod base64_example;
mod getrandom_example;
mod hex_example;
mod subtle;
mod zeroize;

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
    /// Runs the `subtle` example.
    #[command(name = "subtle")]
    Subtle,
    /// Runs the `zeroize` example.
    #[command(name = "zeroize")]
    Zeroize,
    /// Runs the `getrandom` example.
    Getrandom,
    /// Runs the `base64` example.
    Base64,
    /// Runs the `hex` example.
    Hex,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Subtle => {
                subtle::run();
            }
            Commands::Zeroize => {
                zeroize::run();
            }
            Commands::Getrandom => getrandom_example::run()?,
            Commands::Base64 => base64_example::run()?,
            Commands::Hex => hex_example::run()?,
        }
    }
    Ok(())
}
