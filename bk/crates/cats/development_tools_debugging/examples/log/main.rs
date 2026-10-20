use clap::Parser;
use clap::Subcommand;

mod log_custom;
mod log_custom_logger;
mod log_debug;
mod log_env_variable;
mod log_error;
mod log_mod;
mod log_stdout;
#[cfg(target_os = "linux")]
mod log_syslog;
mod log_timestamp;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "log_custom")]
    Custom,
    #[command(name = "log_custom_logger")]
    CustomLogger,
    #[command(name = "log_debug")]
    Debug,
    #[command(name = "log_env_variable")]
    EnvVariable,
    #[command(name = "log_error")]
    Error,
    #[command(name = "log_mod")]
    Mod,
    #[command(name = "log_stdout")]
    Stdout,
    #[cfg(target_os = "linux")]
    #[command(name = "log_syslog")]
    Syslog,
    #[command(name = "log_timestamp")]
    Timestamp,
}

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
            #[cfg(target_os = "linux")]
            Commands::Syslog => {
                log_syslog::run();
            }
            Commands::Timestamp => {
                log_timestamp::run();
            }
        }
    }

    Ok(())
}
