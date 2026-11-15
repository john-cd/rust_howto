//! Runs one of the `ml` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Train a linear model on synthetic data with Candle on the CPU.
#[cfg(feature = "candle")]
mod candle;

/// Cluster two groups of synthetic points with seeded Linfa K-means.
mod kmeans;

/// Predict numeric values with a linear regression model from `linfa-linear`.
mod linfa;

/// Train a SmartCore decision-tree classifier on synthetic data and measure its
mod smartcore;

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
    /// Runs the `candle` example.
    #[cfg(feature = "candle")]
    #[command(name = "candle")]
    Candle,

    /// Runs the `kmeans` example.
    #[command(name = "kmeans")]
    Kmeans,

    /// Runs the `linfa` regression example.
    #[command(name = "linfa")]
    Linfa,

    /// Runs the `smartcore` example.
    #[command(name = "smartcore")]
    Smartcore,
}

/// Runs the example selected on the command line.
fn main() {
    dispatch(Cli::parse());
}

/// Runs the example selected in the parsed command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "candle")]
            Commands::Candle => {
                candle::run();
            }
            Commands::Kmeans => {
                kmeans::run();
            }
            Commands::Linfa => {
                linfa::run();
            }
            Commands::Smartcore => {
                smartcore::run();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory as _;

    use super::*;

    /// Confirms that the command and its arguments are valid.
    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Every declared subcommand name is accepted.
    #[test]
    fn declared_subcommands_parse() {
        let command = Cli::command();
        let names: Vec<_> = command
            .get_subcommands()
            .map(clap::Command::get_name)
            .collect();
        for name in names {
            assert!(
                Cli::try_parse_from(["example", name]).is_ok(),
                "subcommand `{name}` should parse"
            );
        }
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
