//! Runs one of the `trait_objects` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod dyn_autotraits;
mod dyn_compat;
mod dyn_supertraits;
mod trait_objects;

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
    /// Runs the `dyn_autotraits` example.
    #[command(name = "dyn_autotraits")]
    DynAutotraits,
    /// Runs the `dyn_compat` example.
    #[command(name = "dyn_compat")]
    DynCompat,
    /// Runs the `dyn_supertraits` example.
    #[command(name = "dyn_supertraits")]
    DynSupertraits,
    /// Runs the `trait_objects` example.
    #[command(name = "trait_objects")]
    TraitObjects,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::DynAutotraits => {
                dyn_autotraits::run();
            }
            Commands::DynCompat => {
                dyn_compat::run();
            }
            Commands::DynSupertraits => {
                dyn_supertraits::run();
            }
            Commands::TraitObjects => {
                trait_objects::run();
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
    const SUBCOMMANDS: &[&str] = &[
        "dyn_autotraits",
        "dyn_compat",
        "dyn_supertraits",
        "trait_objects",
    ];

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
        let cli = Cli::try_parse_from(["example"])
            .expect("no subcommand is accepted");
        assert!(cli.command.is_none());
        dispatch(cli);
    }
}
