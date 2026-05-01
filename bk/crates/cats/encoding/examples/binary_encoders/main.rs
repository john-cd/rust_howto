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

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Bincode => {
                let _ = bincode::run();
            }
            #[cfg(target_os = "linux")]
            Commands::Capnp => {
                let _ = capnp::run();
            }
            Commands::Ciborium => {
                let _ = ciborium::run();
            }
            Commands::Flatbuffers => {
                let _ = flatbuffers::run();
            }
            Commands::Prost => {
                let _ = prost::run();
            }
            Commands::Protobuf => {
                let _ = protobuf::run();
            }
            Commands::RmpSerde => {
                let _ = rmp_serde::run();
            }
        }
    }
}
