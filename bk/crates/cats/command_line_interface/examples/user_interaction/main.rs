//! Runs one of the `user_interaction` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod indicatif;
mod indicatif2;
mod inquire;

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
    /// Runs the `indicatif` example.
    #[command(name = "indicatif")]
    Indicatif,
    /// Runs the `indicatif2` example.
    #[command(name = "indicatif2")]
    Indicatif2,
    /// Runs the `inquire` example.
    #[command(name = "inquire")]
    Inquire,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Indicatif => {
                indicatif::run()?;
            }
            Commands::Indicatif2 => {
                indicatif2::run()?;
            }
            Commands::Inquire => {
                inquire::run();
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
