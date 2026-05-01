use clap::Parser;
use clap::Subcommand;

mod log_custom;
mod log_custom_logger;
mod log_debug;
mod log_env_variable;
mod log_error;
mod log_mod;
mod log_stdout;
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
    LogCustom,
    #[command(name = "log_custom_logger")]
    LogCustomLogger,
    #[command(name = "log_debug")]
    LogDebug,
    #[command(name = "log_env_variable")]
    LogEnvVariable,
    #[command(name = "log_error")]
    LogError,
    #[command(name = "log_mod")]
    LogMod,
    #[command(name = "log_stdout")]
    LogStdout,
    #[command(name = "log_syslog")]
    LogSyslog,
    #[command(name = "log_timestamp")]
    LogTimestamp,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::LogCustom => {
                log_custom::run();
            }
            Commands::LogCustomLogger => {
                log_custom_logger::run();
            }
            Commands::LogDebug => {
                log_debug::run();
            }
            Commands::LogEnvVariable => {
                log_env_variable::run();
            }
            Commands::LogError => {
                log_error::run();
            }
            Commands::LogMod => {
                log_mod::run();
            }
            Commands::LogStdout => {
                log_stdout::run();
            }
            Commands::LogSyslog => {
                log_syslog::run();
            }
            Commands::LogTimestamp => {
                log_timestamp::run();
            }
        }
    }
}
