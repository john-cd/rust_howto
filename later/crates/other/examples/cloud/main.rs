//! Runs one of the `cloud` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

mod aws_cloudwatch;
mod aws_lambda;
mod aws_lambda_invoke;
mod aws_message_bus;
mod aws_s3_compatible;
mod aws_sdk;
mod aws_ses;
mod aws_shared_config;
mod kubernetes;
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
    /// Runs the in-memory message bus example (no AWS calls).
    #[command(name = "aws_message_bus")]
    AwsMessageBus,
    /// Prints the region of clients built from one shared configuration.
    #[command(name = "aws_shared_config")]
    AwsSharedConfig,
    /// Runs the `aws_sdk` example.
    #[command(name = "aws_sdk")]
    AwsSdk,
    /// Runs the `kubernetes` example.
    #[command(name = "kubernetes")]
    Kubernetes,
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
            Commands::AwsMessageBus => aws_message_bus::run()?,
            Commands::AwsSharedConfig => aws_shared_config::run()?,
            Commands::AwsSdk => aws_sdk::run()?,
            Commands::Kubernetes => kubernetes::run()?,
            Commands::Shuttle => shuttle::run(),
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
