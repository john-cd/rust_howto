// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod multithreading_scoped_threads;
mod multithreading_spawn_join;

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
    /// Runs the `multithreading_scoped_threads` example.
    #[command(name = "multithreading_scoped_threads")]
    MultithreadingScopedThreads,
    /// Runs the `multithreading_spawn_join` example.
    #[command(name = "multithreading_spawn_join")]
    MultithreadingSpawnJoin,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::MultithreadingScopedThreads => {
                multithreading_scoped_threads::run();
            }
            Commands::MultithreadingSpawnJoin => {
                multithreading_spawn_join::run();
            }
        }
    }
}
