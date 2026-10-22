// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

#[cfg(target_family = "unix")]
mod continuous;
#[cfg(target_family = "unix")]
mod error_file;
#[cfg(target_family = "unix")]
mod piped;
mod process_output;
#[cfg(target_family = "unix")]
mod read_env_variable;
#[cfg(target_family = "unix")]
mod send_input;
mod which;

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
    #[cfg(target_family = "unix")]
    #[command(name = "continuous")]
    Continuous,
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
    /// Runs the `which` example.
    #[command(name = "which")]
    Which,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(target_family = "unix")]
            Commands::Continuous => {
                continuous::run();
            }
            #[cfg(target_family = "unix")]
            Commands::ErrorFile => {
                error_file::run();
            }
            #[cfg(target_family = "unix")]
            Commands::Piped => {
                piped::run();
            }
            Commands::ProcessOutput => {
                process_output::run()?;
            }
            #[cfg(target_family = "unix")]
            Commands::ReadEnvVariable => {
                read_env_variable::run();
            }
            #[cfg(target_family = "unix")]
            Commands::SendInput => {
                send_input::run();
            }
            Commands::Which => {
                which::run();
            }
        }
    }

    Ok(())
}
