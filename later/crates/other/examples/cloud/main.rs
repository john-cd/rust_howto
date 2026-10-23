//! Runs one of the `cloud` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

mod aws_lambda;
mod aws_sdk;
mod shuttle;

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
    /// Runs the `aws_lambda` example.
    #[command(name = "aws_lambda")]
    AwsLambda,
    /// Runs the `aws_sdk` example.
    #[command(name = "aws_sdk")]
    AwsSdk,
    /// Runs the `shuttle` example.
    #[command(name = "shuttle")]
    Shuttle,
}

/// Runs the example selected on the command line.
fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AwsLambda => aws_lambda::run()?,
            Commands::AwsSdk => aws_sdk::run()?,
            Commands::Shuttle => shuttle::run(),
        }
    }

    Ok(())
}
