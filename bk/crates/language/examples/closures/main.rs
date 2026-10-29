//! Runs one of the `closures` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod closures;
mod closures_as_input_parameters;
mod closures_capture;
mod closures_move;
mod closures_with_type_annotations;

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
    /// Runs the `closures` example.
    #[command(name = "closures")]
    Closures,
    /// Runs the `closures_as_input_parameters` example.
    #[command(name = "closures_as_input_parameters")]
    ClosuresAsInputParameters,
    /// Runs the `closures_capture` example.
    #[command(name = "closures_capture")]
    ClosuresCapture,
    /// Runs the `closures_move` example.
    #[command(name = "closures_move")]
    ClosuresMove,
    /// Runs the `closures_with_type_annotations` example.
    #[command(name = "closures_with_type_annotations")]
    ClosuresWithTypeAnnotations,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::Closures => {
                closures::run();
            }
            Commands::ClosuresAsInputParameters => {
                closures_as_input_parameters::run();
            }
            Commands::ClosuresCapture => {
                closures_capture::run();
            }
            Commands::ClosuresMove => {
                closures_move::run();
            }
            Commands::ClosuresWithTypeAnnotations => {
                closures_with_type_annotations::run();
            }
        }
    }
}

/// Parses the command line and runs the selected example.
fn main() {
    dispatch(Cli::parse());
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &["closures", "closures_as_input_parameters", "closures_capture", "closures_move", "closures_with_type_annotations"];

    /// The command-line definition is internally consistent.
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

    /// Running without a subcommand selects nothing and succeeds.
    #[test]
    fn test_no_subcommand_runs_nothing() {
        let cli = Cli::try_parse_from(["example"]).expect("no subcommand is accepted");
        assert!(cli.command.is_none());
        dispatch(cli);
    }
}
