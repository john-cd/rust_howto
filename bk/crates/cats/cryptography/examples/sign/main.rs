//! Runs one of the `sign` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod dsa;
mod ecdsa;
mod ed25519;
mod ed25519_dalek;
mod hmac;

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
    /// Runs the `dsa` example.
    #[command(name = "dsa")]
    Dsa,
    /// Runs the `ecdsa` example.
    #[command(name = "ecdsa")]
    Ecdsa,
    /// Runs the `ed25519` example.
    #[command(name = "ed25519")]
    Ed25519,
    /// Runs the `ed25519_dalek` example.
    #[command(name = "ed25519_dalek")]
    Ed25519Dalek,
    /// Runs the `hmac` example.
    #[command(name = "hmac")]
    Hmac,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Dsa => {
                dsa::run();
            }
            Commands::Ecdsa => {
                ecdsa::run()?;
            }
            Commands::Ed25519 => {
                ed25519::run()?;
            }
            Commands::Ed25519Dalek => {
                ed25519_dalek::run()?;
            }
            Commands::Hmac => {
                hmac::run().map_err(|e| anyhow::anyhow!("{e:?}"))?;
            }
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
