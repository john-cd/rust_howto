//! Runs one of the `control_flow` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod for1;
mod if_else;
mod labeled_block_expression;
mod loop1;
mod while1;

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
    /// Runs the `for1` example.
    #[command(name = "for1")]
    For1,
    /// Runs the `if_else` example.
    #[command(name = "if_else")]
    IfElse,
    /// Runs the `labeled_block_expression` example.
    #[command(name = "labeled_block_expression")]
    LabeledBlockExpression,
    /// Runs the `loop1` example.
    #[command(name = "loop1")]
    Loop1,
    /// Runs the `while1` example.
    #[command(name = "while1")]
    While1,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::For1 => {
                for1::run();
            }
            Commands::IfElse => {
                if_else::run();
            }
            Commands::LabeledBlockExpression => {
                labeled_block_expression::run();
            }
            Commands::Loop1 => {
                loop1::run();
            }
            Commands::While1 => {
                while1::run();
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
