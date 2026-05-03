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

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "bincode")]
    Bincode,
    #[cfg(target_os = "linux")]
    #[command(name = "capnp")]
    Capnp,
    #[command(name = "ciborium")]
    Ciborium,
    #[command(name = "flatbuffers")]
    Flatbuffers,
    #[command(name = "prost")]
    Prost,
    #[command(name = "protobuf")]
    Protobuf,
    #[command(name = "rmp_serde")]
    RmpSerde,
}

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
