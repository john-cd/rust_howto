//! Runs one of the hashing examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod blake3;
mod foldhash;
mod md5;
mod sha1;
mod sha2;
mod sha_digest;
mod xxh3_checksum;

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
    /// Runs the `blake3` example.
    #[command(name = "blake3")]
    Blake3,
    /// Runs the `foldhash` example.
    #[command(name = "foldhash")]
    Foldhash,
    /// Runs the `md5` example.
    #[command(name = "md5")]
    Md5,
    /// Runs the `sha1` example.
    #[command(name = "sha1")]
    Sha1,
    /// Runs the `sha2` example.
    #[command(name = "sha2")]
    Sha2,
    /// Runs the `sha_digest` example.
    #[command(name = "sha_digest")]
    ShaDigest,
    /// Runs the `xxh3_checksum` example.
    #[command(name = "xxh3_checksum")]
    Xxh3Checksum,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(command) = cli.command {
        match command {
            Commands::Blake3 => {
                blake3::run();
            }
            Commands::Foldhash => foldhash::run(),
            Commands::Md5 => {
                md5::run();
            }
            Commands::Sha1 => {
                sha1::run();
            }
            Commands::Sha2 => {
                sha2::run()?;
            }
            Commands::ShaDigest => {
                sha_digest::run()?;
            }
            Commands::Xxh3Checksum => {
                xxh3_checksum::run();
            }
        }
    }

    Ok(())
}

/// Parses the command line and runs the selected example.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dispatch(Cli::parse())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &[
        "blake3",
        "foldhash",
        "md5",
        "sha1",
        "sha2",
        "sha_digest",
        "xxh3_checksum",
    ];

    /// The command-line definition is internally consistent.
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

    /// Running without a subcommand selects nothing and succeeds.
    #[test]
    fn test_no_subcommand_runs_nothing() {
        let cli = Cli::try_parse_from(["example"])
            .expect("no subcommand is accepted");
        assert!(cli.command.is_none());
        dispatch(cli).expect("dispatching should succeed");
    }
}
