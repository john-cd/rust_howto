//! Runs the metrics examples.

use clap::Parser;
use clap::Subcommand;

mod prometheus;
mod record_metrics;

/// Command-line arguments for the metrics examples.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Commands,
}

/// The metrics examples available from this target.
#[derive(Subcommand)]
enum Commands {
    /// Records a counter and a histogram.
    Record,
    /// Renders metrics in Prometheus exposition format.
    Prometheus,
}

/// Runs the selected metrics example.
fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Commands::Record => record_metrics::run(),
        Commands::Prometheus => prometheus::run(),
    }
}
