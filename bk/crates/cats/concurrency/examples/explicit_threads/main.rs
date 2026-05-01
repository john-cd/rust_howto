use clap::Parser;
use clap::Subcommand;

mod multithreading_scoped_threads;
mod multithreading_spawn_join;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "multithreading_scoped_threads")]
    MultithreadingScopedThreads,
    #[command(name = "multithreading_spawn_join")]
    MultithreadingSpawnJoin,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::MultithreadingScopedThreads => {
                let _ = multithreading_scoped_threads::run();
            }
            Commands::MultithreadingSpawnJoin => {
                let _ = multithreading_spawn_join::run();
            }
        }
    }
}
