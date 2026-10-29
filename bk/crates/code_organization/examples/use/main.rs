//! Runs one of the `use` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod pub_use;
mod use1;
mod use2;
mod use3;
mod use_external_crate;
mod use_shortcuts;

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
    /// Runs the `pub_use` example.
    #[command(name = "pub_use")]
    PubUse,
    /// Runs the `use1` example.
    #[command(name = "use1")]
    Use1,
    /// Runs the `use2` example.
    #[command(name = "use2")]
    Use2,
    /// Runs the `use3` example.
    #[command(name = "use3")]
    Use3,
    /// Runs the `use_external_crate` example.
    #[command(name = "use_external_crate")]
    UseExternalCrate,
    /// Runs the `use_shortcuts` example.
    #[command(name = "use_shortcuts")]
    UseShortcuts,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::PubUse => {
                pub_use::run();
            }
            Commands::Use1 => {
                use1::run();
            }
            Commands::Use2 => {
                use2::run();
            }
            Commands::Use3 => {
                use3::run();
            }
            Commands::UseExternalCrate => {
                use_external_crate::run()?;
            }
            Commands::UseShortcuts => {
                use_shortcuts::run();
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
    const SUBCOMMANDS: &[&str] = &["pub_use", "use1", "use2", "use3", "use_external_crate", "use_shortcuts"];

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
        dispatch(cli).expect("dispatching should succeed");
    }
}
