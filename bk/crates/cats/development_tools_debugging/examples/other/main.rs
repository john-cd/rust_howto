use clap::Parser;
use clap::Subcommand;

mod open_observe;
mod open_telemetry;
mod slog;
mod type_name_of_val;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "open_observe")]
    OpenObserve,
    #[command(name = "open_telemetry")]
    OpenTelemetry,
    #[command(name = "slog")]
    Slog,
    #[command(name = "type_name_of_val")]
    TypeNameOfVal,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::OpenObserve => {
                let _ = open_observe::run();
            }
            Commands::OpenTelemetry => {
                let _ = open_telemetry::run();
            }
            Commands::Slog => {
                let _ = slog::run();
            }
            Commands::TypeNameOfVal => {
                let _ = type_name_of_val::run();
            }
        }
    }
}
