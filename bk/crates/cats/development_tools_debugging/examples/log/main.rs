//! Runs one of the `log` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod log_custom;
mod log_custom_logger;
mod log_debug;
mod log_env_variable;
mod log_error;
mod log_mod;
mod log_stdout;
#[cfg(unix)]
mod log_syslog;
mod log_timestamp;

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
    /// Runs the `log_custom` example.
    #[command(name = "log_custom")]
    Custom,
    /// Runs the `log_custom_logger` example.
    #[command(name = "log_custom_logger")]
    CustomLogger,
    /// Runs the `log_debug` example.
    #[command(name = "log_debug")]
    Debug,
    /// Runs the `log_env_variable` example.
    #[command(name = "log_env_variable")]
    EnvVariable,
    /// Runs the `log_error` example.
    #[command(name = "log_error")]
    Error,
    /// Runs the `log_mod` example.
    #[command(name = "log_mod")]
    Mod,
    /// Runs the `log_stdout` example.
    #[command(name = "log_stdout")]
    Stdout,
    /// Runs the `log_syslog` example.
    #[cfg(unix)]
    #[command(name = "log_syslog")]
    Syslog,
    /// Runs the `log_timestamp` example.
    #[command(name = "log_timestamp")]
    Timestamp,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Custom => {
                log_custom::run()?;
            }
            Commands::CustomLogger => {
                log_custom_logger::run()?;
            }
            Commands::Debug => {
                log_debug::run();
            }
            Commands::EnvVariable => {
                log_env_variable::run();
            }
            Commands::Error => {
                log_error::run();
            }
            Commands::Mod => {
                log_mod::run();
            }
            Commands::Stdout => {
                log_stdout::run();
            }
            #[cfg(unix)]
            Commands::Syslog => {
                log_syslog::run()?;
            }
            Commands::Timestamp => {
                log_timestamp::run();
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
    const SUBCOMMANDS: &[&str] = &[
        "log_custom",
        "log_custom_logger",
        "log_debug",
        "log_env_variable",
        "log_error",
        "log_mod",
        "log_stdout",
        "log_syslog",
        "log_timestamp",
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
        dispatch(cli).expect("dispatching should succeed");
    }
}
