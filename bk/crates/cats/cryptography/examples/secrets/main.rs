//! Runs one of the `secrets` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod keyring;
mod redacted_secret;
mod sealed_envelope;
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
    /// Runs the `redacted_secret` example.
    #[command(name = "redacted_secret")]
    RedactedSecret,
    /// Runs the `zeroize` example.
    #[command(name = "zeroize")]
    Zeroize,
    /// Runs the `keyring` example.
    #[command(name = "keyring")]
    Keyring,
    /// Runs the `sealed_envelope` example.
    #[command(name = "sealed_envelope")]
    SealedEnvelope,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::RedactedSecret => redacted_secret::run(),
            Commands::Zeroize => zeroize::run(),
            Commands::Keyring => keyring::run(),
            Commands::SealedEnvelope => sealed_envelope::run(),
        }
    }
}
