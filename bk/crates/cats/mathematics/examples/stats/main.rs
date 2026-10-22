// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod central_tendency;
mod central_tendency1;
mod central_tendency2;
mod standard_deviation;

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
    /// Runs the `central_tendency` example.
    #[command(name = "central_tendency")]
    CentralTendency,
    /// Runs the `central_tendency1` example.
    #[command(name = "central_tendency1")]
    CentralTendency1,
    /// Runs the `central_tendency2` example.
    #[command(name = "central_tendency2")]
    CentralTendency2,
    /// Runs the `standard_deviation` example.
    #[command(name = "standard_deviation")]
    StandardDeviation,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CentralTendency => {
                central_tendency::run();
            }
            Commands::CentralTendency1 => {
                central_tendency1::run();
            }
            Commands::CentralTendency2 => {
                central_tendency2::run();
            }
            Commands::StandardDeviation => {
                standard_deviation::run();
            }
        }
    }
}
