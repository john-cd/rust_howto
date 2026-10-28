//! Runs one of the `external_commands` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

#[cfg(target_os = "linux")]
mod continuous;
mod duct_example;
#[cfg(target_family = "unix")]
mod error_file;
#[cfg(target_family = "unix")]
mod piped;
mod process_output;
#[cfg(target_family = "unix")]
mod read_env_variable;
#[cfg(target_family = "unix")]
mod send_input;
mod shlex_example;
mod which;
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
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
