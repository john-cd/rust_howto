// TODO Reviewed ??-??-2026
#[cfg(any(feature = "sled", feature = "surrealdb"))]
use clap::Parser;
#[cfg(any(feature = "sled", feature = "surrealdb"))]
use clap::Subcommand;

#[cfg(feature = "sled")]
mod sled;
#[cfg(feature = "surrealdb")]
mod surrealdb;

/// Command-line arguments.
#[cfg(any(feature = "sled", feature = "surrealdb"))]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(any(feature = "sled", feature = "surrealdb"))]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `sled` example.
    #[cfg(feature = "sled")]
    #[command(name = "sled")]
    Sled,
    /// Runs the `surrealdb` example.
    #[cfg(feature = "surrealdb")]
    #[command(name = "surrealdb")]
    Surrealdb,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    #[cfg(any(feature = "sled", feature = "surrealdb"))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "sled")]
                Commands::Sled => {
                    sled::run()?;
                }
                #[cfg(feature = "surrealdb")]
                Commands::Surrealdb => {
                    surrealdb::run()?;
                }
            }
        }
    }

    Ok(())
}
