//! Runs one of the `structs` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod common_traits;
mod generic_structs;
mod struct_constant;
mod struct_fields;
mod struct_impl;
mod struct_init;
mod struct_trait;
mod struct_update;
mod structs;
mod tuple_structs;

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
    /// Runs the `common_traits` example.
    #[command(name = "common_traits")]
    CommonTraits,
    /// Runs the `generic_structs` example.
    #[command(name = "generic_structs")]
    GenericStructs,
    /// Runs the `struct_constant` example.
    #[command(name = "struct_constant")]
    StructConstant,
    /// Runs the `struct_fields` example.
    #[command(name = "struct_fields")]
    StructFields,
    /// Runs the `struct_impl` example.
    #[command(name = "struct_impl")]
    StructImpl,
    /// Runs the `struct_init` example.
    #[command(name = "struct_init")]
    StructInit,
    /// Runs the `struct_trait` example.
    #[command(name = "struct_trait")]
    StructTrait,
    /// Runs the `struct_update` example.
    #[command(name = "struct_update")]
    StructUpdate,
    /// Runs the `structs` example.
    #[command(name = "structs")]
    Structs,
    /// Runs the `tuple_structs` example.
    #[command(name = "tuple_structs")]
    TupleStructs,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::CommonTraits => {
                common_traits::run();
            }
            Commands::GenericStructs => {
                generic_structs::run();
            }
            Commands::StructConstant => {
                struct_constant::run();
            }
            Commands::StructFields => {
                struct_fields::run();
            }
            Commands::StructImpl => {
                struct_impl::run();
            }
            Commands::StructInit => {
                struct_init::run();
            }
            Commands::StructTrait => {
                struct_trait::run();
            }
            Commands::StructUpdate => {
                struct_update::run();
            }
            Commands::Structs => {
                structs::run();
            }
            Commands::TupleStructs => {
                tuple_structs::run();
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
    const SUBCOMMANDS: &[&str] = &["common_traits", "generic_structs", "struct_constant", "struct_fields", "struct_impl", "struct_init", "struct_trait", "struct_update", "structs", "tuple_structs"];

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
