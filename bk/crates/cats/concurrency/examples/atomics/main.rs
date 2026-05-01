use clap::Parser;
use clap::Subcommand;

mod atomic_cell;
mod atomics;
mod spinlock;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "atomic_cell")]
    AtomicCell,
    #[command(name = "atomics")]
    Atomics,
    #[command(name = "spinlock")]
    Spinlock,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AtomicCell => {
                let _ = atomic_cell::run();
            }
            Commands::Atomics => {
                let _ = atomics::run();
            }
            Commands::Spinlock => {
                let _ = spinlock::run();
            }
        }
    }
}
