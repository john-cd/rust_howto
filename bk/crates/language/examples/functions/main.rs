//! Runs one of the `functions` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod diverging_functions;
mod function_pointers;
mod functions;
mod generic_functions;
mod generic_functions2;
mod generic_functions3;
mod return_reference;

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
    /// Runs the `diverging_functions` example.
    #[command(name = "diverging_functions")]
    DivergingFunctions,
    /// Runs the `function_pointers` example.
    #[command(name = "function_pointers")]
    FunctionPointers,
    /// Runs the `functions` example.
    #[command(name = "functions")]
    Functions,
    /// Runs the `generic_functions` example.
    #[command(name = "generic_functions")]
    GenericFunctions,
    /// Runs the `generic_functions2` example.
    #[command(name = "generic_functions2")]
    GenericFunctions2,
    /// Runs the `generic_functions3` example.
    #[command(name = "generic_functions3")]
    GenericFunctions3,
    /// Runs the `return_reference` example.
    #[command(name = "return_reference")]
    ReturnReference,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::DivergingFunctions => {
                diverging_functions::run();
            }
            Commands::FunctionPointers => {
                function_pointers::run();
            }
            Commands::Functions => {
                functions::run();
            }
            Commands::GenericFunctions => {
                generic_functions::run();
            }
            Commands::GenericFunctions2 => {
                generic_functions2::run();
            }
            Commands::GenericFunctions3 => {
                generic_functions3::run();
            }
            Commands::ReturnReference => {
                return_reference::run();
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
