//! Runs one of the `tls` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod native_tls;
mod rustls;

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
    /// Runs the `native_tls` example.
    #[command(name = "native_tls")]
    NativeTls,
    /// Runs the `rustls` example.
    #[command(name = "rustls")]
    Rustls,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::NativeTls => {
                native_tls::run()?;
            }
            Commands::Rustls => {
                rustls::run()?;
            }
        }
    }

    Ok(())
}
