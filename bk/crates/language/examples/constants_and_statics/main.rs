//! Runs one of the `constants_and_statics` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod constants;
mod static_mut;
mod statics;

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
    /// Runs the `constants` example.
    #[command(name = "constants")]
    Constants,
    /// Runs the `static_mut` example.
    #[command(name = "static_mut")]
    StaticMut,
    /// Runs the `statics` example.
    #[command(name = "statics")]
    Statics,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Constants => {
                constants::run();
            }
            Commands::StaticMut => {
                static_mut::run();
            }
            Commands::Statics => {
                statics::run();
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
