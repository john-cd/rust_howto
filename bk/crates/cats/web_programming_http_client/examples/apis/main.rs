//! Runs one of the `apis` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod paginated;
mod rate_limited;
mod rest_get;
mod rest_head;
mod rest_post;

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
    /// Runs the `paginated` example.
    #[command(name = "paginated")]
    Paginated,
    /// Runs the `rate_limited` example.
    #[command(name = "rate_limited")]
    RateLimited,
    /// Runs the `rest_get` example.
    #[command(name = "rest_get")]
    RestGet,
    /// Runs the `rest_head` example.
    #[command(name = "rest_head")]
    RestHead,
    /// Runs the `rest_post` example.
    #[command(name = "rest_post")]
    RestPost,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Paginated => {
                paginated::run()?;
            }
            Commands::RateLimited => {
                rate_limited::run()?;
            }
            Commands::RestGet => {
                rest_get::run().map_err(|e| anyhow::anyhow!(e.to_string()))?;
            }
            Commands::RestHead => {
                rest_head::run()?;
            }
            Commands::RestPost => {
                rest_post::run()?;
            }
        }
    }

    Ok(())
}
