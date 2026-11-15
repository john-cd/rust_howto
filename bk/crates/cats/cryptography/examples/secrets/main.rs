//! Runs one of the `secrets` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Stores, loads and deletes a secret in the operating-system credential store (Windows Credential
/// Manager, macOS Keychain or the Secret Service) with `keyring`.
mod keyring;

/// Keeps a secret out of logs with a newtype whose `Debug` and `Display`
mod redacted_secret;

/// Seals a secret under a passphrase: Argon2id derives a key, and XChaCha20-Poly1305 encrypts the
/// secret, authenticating a version label and context as associated data.
mod sealed_envelope;

/// Wipes a secret from memory when it is dropped, using `zeroize`'s derive
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
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::RedactedSecret => redacted_secret::run(),
            Commands::Zeroize => zeroize::run(),
            Commands::Keyring => keyring::run(),
            Commands::SealedEnvelope => sealed_envelope::run(),
        }
    }
}

/// Parses the command line and runs the selected example.
fn main() {
    dispatch(Cli::parse());
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &["redacted_secret", "zeroize", "keyring", "sealed_envelope"];

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
