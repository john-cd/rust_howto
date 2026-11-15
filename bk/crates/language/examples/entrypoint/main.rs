//! Runs one of the `entrypoint` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// This is an example of an async main function.
mod async_main;
/// The `main` function, the entry point of a binary crate.
mod main_fn;
/// Example of a `main` function returning a `Result`,
mod main_fn_with_result;

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
    /// Runs the `async_main` example.
    #[command(name = "async_main")]
    AsyncMain,
    /// Runs the `main_fn` example.
    #[command(name = "main_fn")]
    MainFn,
    /// Runs the `main_fn_with_result` example.
    #[command(name = "main_fn_with_result")]
    MainFnWithResult,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::AsyncMain => {
                async_main::run()?;
            }
            Commands::MainFn => {
                main_fn::run();
            }
            Commands::MainFnWithResult => {
                main_fn_with_result::run()?;
            }
        }
    }
    Ok(())
}

/// Parses the command line and runs the selected example.
fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &["async_main", "main_fn", "main_fn_with_result"];

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
