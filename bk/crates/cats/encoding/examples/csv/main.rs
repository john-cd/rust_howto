//! Runs one of the `csv` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Demonstrates how to use a custom delimiter during CSV deserialization.
mod delimiter;
/// This example shows how to filter CSV data based on a query.
mod filter;
/// This example demonstrates invalid data handling when reading a CSV file.
mod invalid;
/// This example shows how to read a CSV file.
mod read;
/// This example demonstrates deserializing CSV records into a struct.
mod read1;
/// Demonstrates serializing structs to CSV using `serde`.
mod serde_serialize;
/// This example shows how to serialize data to a CSV file.
mod serialize;
/// Streams typed CSV records and round-trips a row.
mod stream_and_round_trip;
/// This example shows how to transform CSV data from one format to
mod transform;

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
    /// Runs the `delimiter` example.
    #[command(name = "delimiter")]
    Delimiter,

    /// Runs the `filter` example.
    #[command(name = "filter")]
    Filter,

    /// Runs the `invalid` example.
    #[command(name = "invalid")]
    Invalid,

    /// Runs the `read` example.
    #[command(name = "read")]
    Read,

    /// Runs the `read1` example.
    #[command(name = "read1")]
    Read1,

    /// Runs the `serde_serialize` example.
    #[command(name = "serde_serialize")]
    SerdeSerialize,

    /// Runs the `serialize` example.
    #[command(name = "serialize")]
    Serialize,

    /// Streams typed CSV records and round-trips a row.
    StreamAndRoundTrip,

    /// Runs the `transform` example.
    #[command(name = "transform")]
    Transform,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Delimiter => {
                delimiter::run()?;
            }
            Commands::Filter => {
                filter::run()?;
            }
            Commands::Invalid => {
                invalid::run()?;
            }
            Commands::Read => {
                read::run()?;
            }
            Commands::Read1 => {
                read1::run()?;
            }
            Commands::SerdeSerialize => {
                serde_serialize::run()?;
            }
            Commands::Serialize => {
                serialize::run()?;
            }
            Commands::StreamAndRoundTrip => {
                stream_and_round_trip::run()?;
            }
            Commands::Transform => {
                transform::run()?;
            }
        }
    }
    Ok(())
}

/// Parses the command line and runs the selected example.
fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &[
        "delimiter",
        "filter",
        "invalid",
        "read",
        "read1",
        "serde_serialize",
        "serialize",
        "stream-and-round-trip",
        "transform",
    ];

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
