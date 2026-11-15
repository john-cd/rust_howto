//! Runs one of the `sign` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// This example demonstrates the use of the ECDSA (Elliptic Curve Digital Signature Algorithm) with
/// the P-256 curve for signing and verifying a
mod dsa;

/// Sign and verify a message with ECDSA using the P-256 curve.
mod ecdsa;

/// Create and verify an Ed25519 signature using `ed25519-dalek`.
mod ed25519;

/// This example demonstrates signing and verifying messages with ed25519-dalek.
mod ed25519_dalek;

/// This example demonstrates how to use the `ring` crate to generate an HMAC (Hash-based Message
/// Authentication Code) signature for a message and then
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
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Dsa => {
                dsa::run()?;
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

/// Parses the command line and runs the selected example.
fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &["dsa", "ecdsa", "ed25519", "ed25519_dalek", "hmac"];

    /// Confirms that the command and its arguments are valid.
    #[test]
    fn test_cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Each example is reachable through its own subcommand.
    #[test]
    fn test_every_subcommand_parses() {
        for name in SUBCOMMANDS {
            let cli = Cli::try_parse_from(["example", name])
                .unwrap_or_else(|e| panic!("`{name}` should parse: {e}"));
            assert!(cli.command.is_some(), "`{name}` selected no command");
        }
    }

    /// Subcommand names are unique.
    #[test]
    fn test_subcommand_names_are_unique() {
        let mut sorted = SUBCOMMANDS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), SUBCOMMANDS.len());
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
