//! Runs one of the `lifetimes` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod lifetime_parameter_function;
mod lifetime_parameter_struct;
mod lifetime_parameters;
mod self_referential_struct;
mod self_referential_struct2;
mod shared_reborrows;
mod static_lifetime;

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
    /// Runs the `lifetime_parameter_function` example.
    #[command(name = "lifetime_parameter_function")]
    LifetimeParameterFunction,
    /// Runs the `lifetime_parameter_struct` example.
    #[command(name = "lifetime_parameter_struct")]
    LifetimeParameterStruct,
    /// Runs the `lifetime_parameters` example.
    #[command(name = "lifetime_parameters")]
    LifetimeParameters,
    /// Runs the `shared_reborrows` example.
    #[command(name = "shared_reborrows")]
    SharedReborrows,
    /// Runs the `self_referential_struct` example.
    #[command(name = "self_referential_struct")]
    SelfReferentialStruct,
    /// Runs the `self_referential_struct2` example.
    #[command(name = "self_referential_struct2")]
    SelfReferentialStruct2,
    /// Runs the `static_lifetime` example.
    #[command(name = "static_lifetime")]
    StaticLifetime,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::LifetimeParameterFunction => {
                lifetime_parameter_function::run();
            }
            Commands::LifetimeParameterStruct => {
                lifetime_parameter_struct::run();
            }
            Commands::LifetimeParameters => {
                lifetime_parameters::run();
            }
            Commands::SharedReborrows => {
                shared_reborrows::run();
            }
            Commands::SelfReferentialStruct => {
                self_referential_struct::run();
            }
            Commands::SelfReferentialStruct2 => {
                self_referential_struct2::run();
            }
            Commands::StaticLifetime => {
                static_lifetime::run();
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
        "lifetime_parameter_function",
        "lifetime_parameter_struct",
        "lifetime_parameters",
        "shared_reborrows",
        "self_referential_struct",
        "self_referential_struct2",
        "static_lifetime",
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
