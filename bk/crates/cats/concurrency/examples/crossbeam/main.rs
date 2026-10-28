//! Runs a focused `crossbeam-channel` example from the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod crossbeam_complex;
mod crossbeam_select;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The Crossbeam channel examples.
#[derive(Subcommand)]
enum Commands {
    /// Runs a bounded multi-producer, multi-consumer pipeline.
    #[command(name = "bounded_mpmc")]
    BoundedMpmc,
    /// Selects messages from several channels.
    #[command(name = "select_channels")]
    SelectChannels,
}

/// Runs the selected channel example.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::BoundedMpmc => {
                crossbeam_complex::run()?;
            }
            Commands::SelectChannels => {
                crossbeam_select::run()?;
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
