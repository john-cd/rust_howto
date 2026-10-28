//! Runs one of the `cow` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod cow_alternatives;
mod cow_as_function_param;
mod cow_to_borrowed_owned;
mod function_returning_cow;
mod into_cow;
mod modify_cow_in_place;

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
    /// Runs the `cow_alternatives` example.
    #[command(name = "cow_alternatives")]
    CowAlternatives,
    /// Runs the `cow_as_function_param` example.
    #[command(name = "cow_as_function_param")]
    CowAsFunctionParam,
    /// Runs the `cow_to_borrowed_owned` example.
    #[command(name = "cow_to_borrowed_owned")]
    CowToBorrowedOwned,
    /// Runs the `function_returning_cow` example.
    #[command(name = "function_returning_cow")]
    FunctionReturningCow,
    /// Runs the `into_cow` example.
    #[command(name = "into_cow")]
    IntoCow,
    /// Runs the `modify_cow_in_place` example.
    #[command(name = "modify_cow_in_place")]
    ModifyCowInPlace,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CowAlternatives => {
                cow_alternatives::run();
            }
            Commands::CowAsFunctionParam => {
                cow_as_function_param::run();
            }
            Commands::CowToBorrowedOwned => {
                cow_to_borrowed_owned::run();
            }
            Commands::FunctionReturningCow => {
                function_returning_cow::run();
            }
            Commands::IntoCow => {
                into_cow::run();
            }
            Commands::ModifyCowInPlace => {
                modify_cow_in_place::run();
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
