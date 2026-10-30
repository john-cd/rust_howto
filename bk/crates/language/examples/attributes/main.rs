//! Runs one of the `attributes` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod allow_dead_code;
mod attributes_deprecated;
mod attributes_derive;
mod attributes_early_development;
mod attributes_must_use;
mod attributes_production;
mod cfg_if;
mod conditional_compilation;

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
    /// Runs the `allow_dead_code` example.
    #[command(name = "allow_dead_code")]
    AllowDeadCode,
    /// Runs the `attributes_deprecated` example.
    #[command(name = "attributes_deprecated")]
    AttributesDeprecated,
    /// Runs the `attributes_derive` example.
    #[command(name = "attributes_derive")]
    AttributesDerive,
    /// Runs the `attributes_early_development` example.
    #[command(name = "attributes_early_development")]
    AttributesEarlyDevelopment,
    /// Runs the `attributes_must_use` example.
    #[command(name = "attributes_must_use")]
    AttributesMustUse,
    /// Runs the `attributes_production` example.
    #[command(name = "attributes_production")]
    AttributesProduction,
    /// Runs the `cfg_if` example.
    #[command(name = "cfg_if")]
    CfgIf,
    /// Runs the `conditional_compilation` example.
    #[command(name = "conditional_compilation")]
    ConditionalCompilation,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::AllowDeadCode => {
                allow_dead_code::run();
            }
            Commands::AttributesDeprecated => {
                attributes_deprecated::run();
            }
            Commands::AttributesDerive => {
                attributes_derive::run();
            }
            Commands::AttributesEarlyDevelopment => {
                attributes_early_development::run();
            }
            Commands::AttributesMustUse => {
                attributes_must_use::run();
            }
            Commands::AttributesProduction => {
                attributes_production::run();
            }
            Commands::CfgIf => {
                cfg_if::run();
            }
            Commands::ConditionalCompilation => {
                conditional_compilation::run();
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
        "allow_dead_code",
        "attributes_deprecated",
        "attributes_derive",
        "attributes_early_development",
        "attributes_must_use",
        "attributes_production",
        "cfg_if",
        "conditional_compilation",
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
