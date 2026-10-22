// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod atomic_cell;
mod atomics;
mod spinlock;

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
    /// Runs the `atomic_cell` example.
    #[command(name = "atomic_cell")]
    AtomicCell,
    /// Runs the `atomics` example.
    #[command(name = "atomics")]
    Atomics,
    /// Runs the `spinlock` example.
    #[command(name = "spinlock")]
    Spinlock,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AtomicCell => {
                atomic_cell::run();
            }
            Commands::Atomics => {
                atomics::run();
            }
            Commands::Spinlock => {
                spinlock::run();
            }
        }
    }
}
