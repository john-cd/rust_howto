//! Runs one of the `aead` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod aes_gcm;
mod aes_gcm_siv;
mod chacha20poly1305;

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
    /// Runs the `aes_gcm` example.
    #[command(name = "aes_gcm")]
    AesGcm,
    /// Runs the `aes_gcm_siv` example.
    #[command(name = "aes_gcm_siv")]
    AesGcmSiv,
    /// Runs the `chacha20poly1305` example.
    #[command(name = "chacha20poly1305")]
    Chacha20poly1305,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AesGcm => {
                aes_gcm::run();
            }
            Commands::AesGcmSiv => {
                aes_gcm_siv::run().map_err(|e| anyhow::anyhow!("{e:?}"))?;
            }
            Commands::Chacha20poly1305 => {
                chacha20poly1305::run();
            }
        }
    }

    Ok(())
}
