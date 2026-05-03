use clap::Parser;
use clap::Subcommand;

mod arc_make_mut;
mod global_mut_state;
mod send_sync;
mod shared_state_mutex;
mod shared_state_parking_lot;
mod shared_state_parking_lot2;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "arc_make_mut")]
    ArcMakeMut,
    #[command(name = "global_mut_state")]
    GlobalMutState,
    #[command(name = "send_sync")]
    SendSync,
    #[command(name = "shared_state_mutex")]
    SharedStateMutex,
    #[command(name = "shared_state_parking_lot")]
    SharedStateParkingLot,
    #[command(name = "shared_state_parking_lot2")]
    SharedStateParkingLot2,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::ArcMakeMut => {
                arc_make_mut::run();
            }
            Commands::GlobalMutState => {
                global_mut_state::run()?;
            }
            Commands::SendSync => {
                send_sync::run();
            }
            Commands::SharedStateMutex => {
                shared_state_mutex::run();
            }
            Commands::SharedStateParkingLot => {
                shared_state_parking_lot::run();
            }
            Commands::SharedStateParkingLot2 => {
                shared_state_parking_lot2::run();
            }
        }
    }
    Ok(())
}
