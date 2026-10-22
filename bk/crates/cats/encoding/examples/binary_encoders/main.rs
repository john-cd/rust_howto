//! Runs one of the `binary_encoders` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod bincode;
#[cfg(target_os = "linux")]
mod capnp;
mod ciborium;
mod flatbuffers;
mod prost;
mod protobuf;
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
    #[cfg(target_os = "linux")]
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
    /// Runs the `protobuf` example.
    #[command(name = "protobuf")]
    Protobuf,
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
            #[cfg(target_os = "linux")]
            Commands::Capnp => {
                capnp::run();
            }
            Commands::Ciborium => {
                ciborium::run()?;
            }
            Commands::Flatbuffers => {
                flatbuffers::run();
            }
            Commands::Prost => {
                prost::run();
            }
            Commands::Protobuf => {
                protobuf::run();
            }
            Commands::RmpSerde => {
                rmp_serde::run()?;
            }
        }
    }
    Ok(())
}
