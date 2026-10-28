//! Runs one of the `scheduling` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod cron_loop;
mod cron_next;
mod interval_job;

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
    /// Runs the `cron_loop` example.
    #[command(name = "cron_loop")]
    CronLoop,
    /// Runs the `cron_next` example.
    #[command(name = "cron_next")]
    CronNext,
    /// Runs the `interval_job` example.
    #[command(name = "interval_job")]
    IntervalJob,
}

/// Runs the example selected on the command line and propagates failures.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CronLoop => {
                cron_loop::run()?;
            }
            Commands::CronNext => {
                cron_next::run()?;
            }
            Commands::IntervalJob => {
                interval_job::run()?;
            }
        }
    }
    Ok(())
}
