//! Runs one of the `option` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod option_combinators;
mod option_match;
mod option_question_mark;
mod option_ref;
mod option_unwrap;
mod options1;
mod options2;

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
    /// Runs the `option_combinators` example.
    #[command(name = "option_combinators")]
    OptionCombinators,
    /// Runs the `option_match` example.
    #[command(name = "option_match")]
    OptionMatch,
    /// Runs the `option_question_mark` example.
    #[command(name = "option_question_mark")]
    OptionQuestionMark,
    /// Runs the `option_ref` example.
    #[command(name = "option_ref")]
    OptionRef,
    /// Runs the `option_unwrap` example.
    #[command(name = "option_unwrap")]
    OptionUnwrap,
    /// Runs the `options1` example.
    #[command(name = "options1")]
    Options1,
    /// Runs the `options2` example.
    #[command(name = "options2")]
    Options2,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::OptionCombinators => {
                option_combinators::run();
            }
            Commands::OptionMatch => {
                option_match::run();
            }
            Commands::OptionQuestionMark => {
                option_question_mark::run();
            }
            Commands::OptionRef => {
                option_ref::run();
            }
            Commands::OptionUnwrap => {
                option_unwrap::run();
            }
            Commands::Options1 => {
                options1::run();
            }
            Commands::Options2 => {
                options2::run();
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
