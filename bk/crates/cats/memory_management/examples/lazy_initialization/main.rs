//! Runs one of the `lazy_initialization` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod lazy_cell;
mod lazy_constant;
mod lazy_lock;
mod lazy_static;
mod once_cell;
mod once_cell2;
mod once_lock;

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
    /// Runs the `lazy_cell` example.
    #[command(name = "lazy_cell")]
    LazyCell,
    /// Runs the `lazy_lock` example.
    #[command(name = "lazy_lock")]
    LazyLock,
    /// Runs the `lazy_constant` example.
    #[command(name = "lazy_constant")]
    LazyConstant,
    /// Runs the `lazy_static` example.
    #[command(name = "lazy_static")]
    LazyStatic,
    /// Runs the `once_cell` example.
    #[command(name = "once_cell")]
    OnceCell,
    /// Runs the `once_cell2` example.
    #[command(name = "once_cell2")]
    OnceCell2,
    /// Runs the `once_lock` example.
    #[command(name = "once_lock")]
    OnceLock,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::LazyCell => lazy_cell::run(),
            Commands::LazyLock => lazy_lock::run(),
            Commands::LazyConstant => {
                lazy_constant::run();
            }
            Commands::LazyStatic => {
                lazy_static::run()?;
            }
            Commands::OnceCell => {
                once_cell::run();
            }
            Commands::OnceCell2 => {
                once_cell2::run();
            }
            Commands::OnceLock => once_lock::run(),
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
    const SUBCOMMANDS: &[&str] = &["lazy_cell", "lazy_lock", "lazy_constant", "lazy_static", "once_cell", "once_cell2", "once_lock"];

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
