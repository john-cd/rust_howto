//! Runs one of the `excel` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod calamine;
mod xlsxwriter;

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
    /// Runs the `calamine` example.
    #[command(name = "calamine")]
    Calamine,
    /// Runs the `xlsxwriter` example.
    #[command(name = "xlsxwriter")]
    Xlsxwriter,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Calamine => {
                calamine::run()?;
            }
            Commands::Xlsxwriter => {
                xlsxwriter::run()?;
            }
        }
    }
    Ok(())
}
