//! Runs one of the `external_commands` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// This example demonstrates how to continuously read from the standard output of an external
/// command, filter the output, and print the filtered lines.
#[cfg(target_os = "linux")]
mod continuous;
/// Run a child command with `duct` and capture its output.
mod duct_example;

/// Defines failures emitted by external commands operations and conversions used at component
/// boundaries. Callers match these errors to choose recovery or user-facing diagnostics.
#[cfg(target_family = "unix")]
mod error_file;

/// This example demonstrates how to pipe the output of one external command to
#[cfg(target_family = "unix")]
mod piped;
/// This example demonstrates how to run an external command, capture its
mod process_output;

/// This example demonstrates how to read an environment variable.
#[cfg(target_family = "unix")]
mod read_env_variable;

/// This example demonstrates how to send input to an external command's standard input (`stdin`)
/// and process its standard output (`stdout`) and
#[cfg(target_family = "unix")]
mod send_input;
/// Split and quote command lines with `shlex`.
mod shlex_example;
/// Demonstrates how to use the `which` crate to find the location of
mod which;
/// Run a Rust tool with `xshell` and capture its output.
mod xshell_example;

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
    /// Runs the `continuous` example.
    #[cfg(target_os = "linux")]
    #[command(name = "continuous")]
    Continuous,

    /// Runs the `duct` example.
    #[command(name = "duct")]
    Duct,

    /// Runs the `error_file` example.
    #[cfg(target_family = "unix")]
    #[command(name = "error_file")]
    ErrorFile,

    /// Runs the `piped` example.
    #[cfg(target_family = "unix")]
    #[command(name = "piped")]
    Piped,

    /// Runs the `process_output` example.
    #[command(name = "process_output")]
    ProcessOutput,

    /// Runs the `read_env_variable` example.
    #[cfg(target_family = "unix")]
    #[command(name = "read_env_variable")]
    ReadEnvVariable,

    /// Runs the `send_input` example.
    #[cfg(target_family = "unix")]
    #[command(name = "send_input")]
    SendInput,

    /// Runs the `shlex` example.
    #[command(name = "shlex")]
    Shlex,

    /// Runs the `which` example.
    #[command(name = "which")]
    Which,

    /// Runs the `xshell` example.
    #[command(name = "xshell")]
    Xshell,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            #[cfg(target_os = "linux")]
            Commands::Continuous => continuous::run()?,
            Commands::Duct => duct_example::run()?,
            #[cfg(target_family = "unix")]
            Commands::ErrorFile => error_file::run()?,
            #[cfg(target_family = "unix")]
            Commands::Piped => piped::run()?,
            Commands::ProcessOutput => {
                process_output::run()?;
            }
            #[cfg(target_family = "unix")]
            Commands::ReadEnvVariable => {
                read_env_variable::run();
            }
            #[cfg(target_family = "unix")]
            Commands::SendInput => send_input::run()?,
            Commands::Shlex => shlex_example::run()?,
            Commands::Which => {
                which::run();
            }
            Commands::Xshell => xshell_example::run()?,
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
        #[cfg(target_os = "linux")]
        "continuous",
        "duct",
        #[cfg(target_family = "unix")]
        "error_file",
        #[cfg(target_family = "unix")]
        "piped",
        "process_output",
        #[cfg(target_family = "unix")]
        "read_env_variable",
        #[cfg(target_family = "unix")]
        "send_input",
        "shlex",
        "which",
        "xshell",
    ];

    /// Confirms that the command and its arguments are valid.
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
}
