//! Runs the metrics examples.

use clap::Parser;
use clap::Subcommand;

mod metrics_route;
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
    /// Serves the payload from an Axum `/metrics` route.
    Route,
}

/// Runs the selected metrics example.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Record => record_metrics::run(),
        Commands::Prometheus => prometheus::run(),
        Commands::Route => metrics_route::run(),
    }
}

fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each example is reachable through its own subcommand.
    #[test]
    fn test_every_subcommand_parses() -> anyhow::Result<()> {
        for name in ["record", "prometheus", "route"] {
            Cli::try_parse_from(["example", name])?;
        }
        Ok(())
    }

    /// The `record` example runs offline.
    // Prometheus and Route install a global recorder, so they run in their
    // own modules' tests.
    #[test]
    fn test_record_runs() -> anyhow::Result<()> {
        dispatch(Cli::try_parse_from(["example", "record"])?)
    }

    /// A subcommand is required.
    #[test]
    fn test_subcommand_is_required() {
        assert!(Cli::try_parse_from(["example"]).is_err());
    }
}
