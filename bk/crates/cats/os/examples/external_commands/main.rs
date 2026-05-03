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

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(target_family = "unix")]
    #[command(name = "continuous")]
    Continuous,
    #[cfg(target_family = "unix")]
    #[command(name = "error_file")]
    ErrorFile,
    #[cfg(target_family = "unix")]
    #[command(name = "piped")]
    Piped,
    #[command(name = "process_output")]
    ProcessOutput,
    #[cfg(target_family = "unix")]
    #[command(name = "read_env_variable")]
    ReadEnvVariable,
    #[cfg(target_family = "unix")]
    #[command(name = "send_input")]
    SendInput,
    #[command(name = "which")]
    Which,
}

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
