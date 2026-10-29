//! Runs one of the `string_encoding` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod base64;
mod form_urlencoded;
mod hex;
mod hex_crate;
mod percent_encode;
mod percent_encoding;
mod url_encode;

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
    /// Runs the `base64` example.
    #[command(name = "base64")]
    Base64,
    /// Runs the `form_urlencoded` example.
    #[command(name = "form_urlencoded")]
    FormUrlencoded,
    /// Runs the `hex` example.
    #[command(name = "hex")]
    Hex,
    /// Runs the `hex_crate` example.
    #[command(name = "hex_crate")]
    HexCrate,
    /// Runs the `percent_encode` example.
    #[command(name = "percent_encode")]
    PercentEncode,
    /// Runs the `percent_encoding` example.
    #[command(name = "percent_encoding")]
    PercentEncoding,
    /// Runs the `url_encode` example.
    #[command(name = "url_encode")]
    UrlEncode,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Base64 => {
                base64::run()?;
            }
            Commands::FormUrlencoded => {
                form_urlencoded::run()?;
            }
            Commands::HexCrate => {
                hex_crate::run()?;
            }
            Commands::Hex => {
                hex::run()?;
            }
            Commands::PercentEncode => {
                percent_encode::run()?;
            }
            Commands::PercentEncoding => {
                percent_encoding::run()?;
            }
            Commands::UrlEncode => {
                url_encode::run();
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
    const SUBCOMMANDS: &[&str] = &["base64", "form_urlencoded", "hex", "hex_crate", "percent_encode", "percent_encoding", "url_encode"];

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
        let cli = Cli::try_parse_from(["example"]).expect("no subcommand is accepted");
        assert!(cli.command.is_none());
        dispatch(cli).expect("dispatching should succeed");
    }
}
