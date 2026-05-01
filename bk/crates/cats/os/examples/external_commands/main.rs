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

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(target_family = "unix")]
            Commands::Continuous => {
                let _ = continuous::run();
            }
            #[cfg(target_family = "unix")]
            Commands::ErrorFile => {
                let _ = error_file::run();
            }
            #[cfg(target_family = "unix")]
            Commands::Piped => {
                let _ = piped::run();
            }
            Commands::ProcessOutput => {
                let _ = process_output::run();
            }
            #[cfg(target_family = "unix")]
            Commands::ReadEnvVariable => {
                let _ = read_env_variable::run();
            }
            #[cfg(target_family = "unix")]
            Commands::SendInput => {
                let _ = send_input::run();
            }
            Commands::Which => {
                let _ = which::run();
            }
        }
    }
}
