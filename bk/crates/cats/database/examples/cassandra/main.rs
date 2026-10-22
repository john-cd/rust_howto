//! Runs one of the `cassandra` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

#[cfg(feature = "cassandra")]
use clap::Parser;
#[cfg(feature = "cassandra")]
use clap::Subcommand;

#[cfg(feature = "cassandra")]
mod cassandra_protocol;
#[cfg(feature = "cassandra")]
mod cdrs_tokio;

/// Command-line arguments.
#[cfg(feature = "cassandra")]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(feature = "cassandra")]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `cassandra_protocol` example.
    #[cfg(feature = "cassandra")]
    #[command(name = "cassandra_protocol")]
    CassandraProtocol,
    /// Runs the `cdrs_tokio` example.
    #[cfg(feature = "cassandra")]
    #[command(name = "cdrs_tokio")]
    CdrsTokio,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    run()
}

/// Runs the example selected on the command line.
fn run() -> anyhow::Result<()> {
    #[cfg(feature = "cassandra")]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                Commands::CassandraProtocol => {
                    cassandra_protocol::run()?;
                }
                Commands::CdrsTokio => {
                    let runtime = tokio::runtime::Runtime::new()?;
                    runtime.block_on(cdrs_tokio::run())?;
                }
            }
        }
    }
    Ok(())
}
