//! Runs one of the `creational_patterns` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod abstract_factory;
mod factory;
mod singleton;

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
    /// Runs the `abstract_factory` example.
    #[command(name = "abstract_factory")]
    AbstractFactory,
    /// Runs the `factory` example.
    #[command(name = "factory")]
    Factory,
    /// Runs the `singleton` example.
    #[command(name = "singleton")]
    Singleton,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AbstractFactory => {
                abstract_factory::run();
            }
            Commands::Factory => {
                factory::run();
            }
            Commands::Singleton => {
                singleton::run();
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
