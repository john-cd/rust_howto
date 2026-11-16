//! Runs one of the `error` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Demonstrates chaining fallible operations with `Result::and_then`.
mod and_then;
/// Demonstrates the use of the `anyhow` crate for error handling.
mod anyhow;
/// This example demonstrates how to use backtraces to debug errors.
mod backtrace;
/// Demonstrates how to use the `color-eyre` crate for enhanced error reporting.
mod color_eyre;

/// Defines failures emitted by error operations and conversions used at component boundaries.
/// Callers match these errors to choose recovery or user-facing diagnostics.
mod error_handling;
/// Demonstrates transforming successful values and errors with `Result`.
mod map;
/// This example demonstrates how to define custom error types using `miette` and `thiserror`.
mod miette;
/// This example demonstrates a simple panic.
mod panic;
/// Demonstrates the use of `?`.
mod question_mark;
/// Propagating errors with the `?` operator through a function that parses a
mod question_mark2;
/// Demonstrates returning and handling a recoverable error with `Result`.
mod result;
/// This example demonstrates how to avoid discarding errors during error
mod retain;

/// Defines failures emitted by error operations and conversions used at component boundaries.
/// Callers match these errors to choose recovery or user-facing diagnostics.
mod thiserror;

/// Defines failures emitted by error operations and conversions used at component boundaries.
/// Callers match these errors to choose recovery or user-facing diagnostics.
mod thiserror2;
/// Demonstrates the use of `unwrap()` and `expect()` for handling `Result`
mod unwrap;
/// Demonstrates the use of `unwrap_or` and `unwrap_or_else` for handling
mod unwrap_or_else;

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
    /// Runs the `anyhow` example.
    #[command(name = "anyhow")]
    Anyhow,

    /// Runs the `and_then` example.
    #[command(name = "and_then")]
    AndThen,

    /// Runs the `backtrace` example.
    #[command(name = "backtrace")]
    Backtrace,

    /// Runs the `color_eyre` example.
    #[command(name = "color_eyre")]
    ColorEyre,

    /// Runs the `error_handling` example.
    #[command(name = "error_handling")]
    ErrorHandling,

    /// Runs the `miette` example.
    #[command(name = "miette")]
    Miette,

    /// Runs the `map` example.
    #[command(name = "map")]
    Map,

    /// Runs the `panic` example.
    #[command(name = "panic")]
    Panic,

    /// Runs the `question_mark` example.
    #[command(name = "question_mark")]
    QuestionMark,

    /// Runs the `question_mark2` example.
    #[command(name = "question_mark2")]
    QuestionMark2,

    /// Runs the `retain` example.
    #[command(name = "retain")]
    Retain,

    /// Runs the `Result` example.
    #[command(name = "result")]
    Result,

    /// Runs the `thiserror` example.
    #[command(name = "thiserror")]
    Thiserror,

    /// Runs the `thiserror2` example.
    #[command(name = "thiserror2")]
    Thiserror2,

    /// Runs the `unwrap` example.
    #[command(name = "unwrap")]
    Unwrap,

    /// Runs the `unwrap_or_else` example.
    #[command(name = "unwrap_or_else")]
    UnwrapOrElse,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> ::anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Anyhow => {
                anyhow::run()?;
            }
            Commands::AndThen => {
                and_then::run();
            }
            Commands::Backtrace => {
                backtrace::run().map_err(|e| ::anyhow::anyhow!(e.to_string()))?;
            }
            Commands::ColorEyre => {
                color_eyre::run().map_err(|e| ::anyhow::anyhow!(e.to_string()))?;
            }
            Commands::ErrorHandling => {
                error_handling::run();
            }
            Commands::Miette => {
                miette::run().map_err(|e| ::anyhow::anyhow!(e.to_string()))?;
            }
            Commands::Map => {
                map::run();
            }
            Commands::Panic => {
                panic::run();
            }
            Commands::QuestionMark => {
                question_mark::run();
            }
            Commands::QuestionMark2 => {
                question_mark2::run();
            }
            Commands::Retain => {
                retain::run()?;
            }
            Commands::Result => {
                result::run();
            }
            Commands::Thiserror => {
                thiserror::run().map_err(|e| ::anyhow::anyhow!(e.to_string()))?;
            }
            Commands::Thiserror2 => {
                thiserror2::run();
            }
            Commands::Unwrap => {
                unwrap::run();
            }
            Commands::UnwrapOrElse => {
                unwrap_or_else::run();
            }
        }
    }

    Ok(())
}

/// Parses the command line and runs the selected example.
fn main() -> ::anyhow::Result<()> {
    dispatch(Cli::parse())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &[
        "anyhow",
        "and_then",
        "backtrace",
        "color_eyre",
        "error_handling",
        "miette",
        "map",
        "panic",
        "question_mark",
        "question_mark2",
        "retain",
        "result",
        "thiserror",
        "thiserror2",
        "unwrap",
        "unwrap_or_else",
    ];

    /// Confirms that the command and its arguments are valid.
    #[test]
    fn test_cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Each example is reachable through its own subcommand.
    #[test]
    fn test_every_subcommand_parses() {
        for name in SUBCOMMANDS {
            let cli = Cli::try_parse_from(["example", name])
                .unwrap_or_else(|e| panic!("`{name}` should parse: {e}"));
            assert!(cli.command.is_some(), "`{name}` selected no command");
        }
    }

    /// Subcommand names are unique.
    #[test]
    fn test_subcommand_names_are_unique() {
        let mut sorted = SUBCOMMANDS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), SUBCOMMANDS.len());
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
