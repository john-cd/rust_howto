//! Runs one of the `nosql` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(any(feature = "mongodb", feature = "redis"))]
use clap::Parser;
#[cfg(any(feature = "mongodb", feature = "redis"))]
use clap::Subcommand;

#[cfg(feature = "mongodb")]
mod mongodb;
#[cfg(feature = "redis")]
mod redis;

/// Command-line arguments.
#[cfg(any(feature = "mongodb", feature = "redis"))]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(any(feature = "mongodb", feature = "redis"))]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `mongodb` example.
    #[cfg(feature = "mongodb")]
    #[command(name = "mongodb")]
    Mongodb,
    /// Runs the `redis` example.
    #[cfg(feature = "redis")]
    #[command(name = "redis")]
    Redis,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    #[cfg(any(feature = "mongodb", feature = "redis"))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "mongodb")]
                Commands::Mongodb => {
                    mongodb::run()?;
                }
                #[cfg(feature = "redis")]
                Commands::Redis => {
                    redis::run()?;
                }
            }
        }
    }
    Ok(())
}
