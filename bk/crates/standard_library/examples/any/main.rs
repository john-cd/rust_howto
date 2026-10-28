//! Runs one of the `any` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod any;
mod plugin;
mod type_id;

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
    /// Runs the `any` example.
    #[command(name = "any")]
    Any,
    /// Runs the `plugin` example.
    #[command(name = "plugin")]
    Plugin,
    /// Runs the `type_id` example.
    #[command(name = "type_id")]
    TypeId,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Any => {
                any::run();
            }
            Commands::Plugin => {
                plugin::run();
            }
            Commands::TypeId => {
                type_id::run();
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
