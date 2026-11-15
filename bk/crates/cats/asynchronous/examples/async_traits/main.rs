//! Runs one of the `async_traits` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// A native `async fn` in a trait (AFIT), called through generics.
mod afit;
/// `#[async_trait]` for trait objects.
mod async_trait_dyn;
/// An `async fn` in a trait, implemented by a health checker.
mod async_traits;
/// Return-position `impl Trait` in a trait method.
mod async_traits2;
/// Asynchronous trait methods with the `async_trait` crate.
mod async_traits3;
/// Making the futures of a trait method `Send` so the trait can be used with `tokio::spawn`: either
/// declare `-> impl Future<Output = T> + Send`, or let `trait_variant` generate a `Send` version of
/// the trait.
mod rpitit_send;

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
    /// Runs the `async_traits` example.
    #[command(name = "async_traits")]
    AsyncTraits,
    /// Runs the `async_traits2` example.
    #[command(name = "async_traits2")]
    AsyncTraits2,
    /// Runs the `async_traits3` example.
    #[command(name = "async_traits3")]
    AsyncTraits3,
    /// Runs the `afit` example.
    #[command(name = "afit")]
    Afit,
    /// Runs the `rpitit_send` example.
    #[command(name = "rpitit_send")]
    RpititSend,
    /// Runs the `async_trait_dyn` example.
    #[command(name = "async_trait_dyn")]
    AsyncTraitDyn,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::AsyncTraits => {
                async_traits::run();
            }
            Commands::AsyncTraits2 => {
                async_traits2::run();
            }
            Commands::AsyncTraits3 => {
                async_traits3::run();
            }
            Commands::Afit => {
                afit::run();
            }
            Commands::RpititSend => {
                rpitit_send::run();
            }
            Commands::AsyncTraitDyn => {
                async_trait_dyn::run();
            }
        }
    }
}
/// Runs the example.
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
        "async_traits",
        "async_traits2",
        "async_traits3",
        "afit",
        "rpitit_send",
        "async_trait_dyn",
    ];

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
