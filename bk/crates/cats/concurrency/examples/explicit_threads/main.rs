//! Runs one of the `explicit_threads` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod multithreading_scoped_threads;
mod multithreading_spawn_join;
mod thread_builder;

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
    /// Runs the scoped-thread example.
    #[command(name = "scoped_threads")]
    ScopedThreads,
    /// Runs the spawn-and-join example.
    #[command(name = "spawn_join")]
    SpawnJoin,
    /// Runs the thread-builder example.
    #[command(name = "thread_builder")]
    ThreadBuilder,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::ScopedThreads => {
                multithreading_scoped_threads::run()?;
            }
            Commands::SpawnJoin => {
                multithreading_spawn_join::run()?;
            }
            Commands::ThreadBuilder => {
                thread_builder::run()?;
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
