//! Runs one of the `binary_encoders` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod bincode;
mod capnp;
mod ciborium;
mod flatbuffers;
mod prost;
mod rmp_serde;

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
    /// Runs the `bincode` example.
    #[command(name = "bincode")]
    Bincode,
    /// Runs the `capnp` example.
    #[command(name = "capnp")]
    Capnp,
    /// Runs the `ciborium` example.
    #[command(name = "ciborium")]
    Ciborium,
    /// Runs the `flatbuffers` example.
    #[command(name = "flatbuffers")]
    Flatbuffers,
    /// Runs the `prost` example.
    #[command(name = "prost")]
    Prost,
    /// Runs the `rmp_serde` example.
    #[command(name = "rmp_serde")]
    RmpSerde,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Bincode => {
                bincode::run()?;
            }
            Commands::Capnp => {
                capnp::run()?;
            }
            Commands::Ciborium => {
                ciborium::run()?;
            }
            Commands::Flatbuffers => {
                flatbuffers::run()?;
            }
            Commands::Prost => {
                prost::run()?;
            }
            Commands::RmpSerde => {
                rmp_serde::run()?;
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
    const SUBCOMMANDS: &[&str] = &[
        "bincode",
        "capnp",
        "ciborium",
        "flatbuffers",
        "prost",
        "rmp_serde",
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
