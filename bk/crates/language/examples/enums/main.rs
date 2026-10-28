//! Runs one of the `enums` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod enums;
mod strum;
mod strum_case;
mod strum_messages;
mod strum_props;

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
    /// Runs the `enums` example.
    #[command(name = "enums")]
    Enums,
    /// Runs the `strum` example.
    #[command(name = "strum")]
    Strum,
    /// Runs the `strum_messages` example.
    #[command(name = "strum_messages")]
    StrumMessages,
    /// Runs the `strum_props` example.
    #[command(name = "strum_props")]
    StrumProps,
    /// Runs the `strum_case` example.
    #[command(name = "strum_case")]
    StrumCase,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Enums => {
                enums::run();
            }
            Commands::Strum => {
                strum::run();
            }
            Commands::StrumMessages => {
                strum_messages::run();
            }
            Commands::StrumProps => {
                strum_props::run();
            }
            Commands::StrumCase => {
                strum_case::run();
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
