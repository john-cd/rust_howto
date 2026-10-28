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
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
