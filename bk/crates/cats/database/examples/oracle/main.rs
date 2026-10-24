//! Runs one of the `oracle` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(any(feature = "oracle", feature = "sibyl"))]
use clap::Parser;
#[cfg(any(feature = "oracle", feature = "sibyl"))]
use clap::Subcommand;

#[cfg(feature = "oracle")]
mod diesel_oci;
#[cfg(feature = "oracle")]
mod oracle;
#[cfg(feature = "sibyl")]
mod sibyl;

/// Command-line arguments.
#[cfg(any(feature = "oracle", feature = "sibyl"))]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(any(feature = "oracle", feature = "sibyl"))]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `diesel_oci` example.
    #[cfg(feature = "oracle")]
    #[command(name = "diesel_oci")]
    DieselOci,
    /// Runs the `oracle` example.
    #[cfg(feature = "oracle")]
    #[command(name = "oracle")]
    Oracle,
    /// Runs the `sibyl` example.
    #[cfg(feature = "sibyl")]
    #[command(name = "sibyl")]
    Sibyl,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    #[cfg(any(feature = "oracle", feature = "sibyl"))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "oracle")]
                Commands::DieselOci => {
                    diesel_oci::run()?;
                }
                #[cfg(feature = "oracle")]
                Commands::Oracle => {
                    oracle::run().map_err(anyhow::Error::msg)?;
                }
                #[cfg(feature = "sibyl")]
                Commands::Sibyl => {
                    sibyl::run()?;
                }
            }
        }
    }
    Ok(())
}
