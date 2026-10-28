//! Runs one of the `trigonometry` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod latitude_longitude;
mod side_length;
mod tan_sin_cos;

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
    /// Runs the `latitude_longitude` example.
    #[command(name = "latitude_longitude")]
    LatitudeLongitude,
    /// Runs the `side_length` example.
    #[command(name = "side_length")]
    SideLength,
    /// Runs the `tan_sin_cos` example.
    #[command(name = "tan_sin_cos")]
    TanSinCos,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::LatitudeLongitude => {
                latitude_longitude::run();
            }
            Commands::SideLength => {
                side_length::run();
            }
            Commands::TanSinCos => {
                tan_sin_cos::run();
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
