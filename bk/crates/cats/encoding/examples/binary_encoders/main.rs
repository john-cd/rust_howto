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
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
