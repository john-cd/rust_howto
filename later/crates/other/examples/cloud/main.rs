//! Runs one of the `cloud` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

/// Publish a custom metric with dimensions to Amazon CloudWatch.
mod aws_cloudwatch;
/// This is a basic example of an AWS Lambda function written in Rust.
mod aws_lambda;
/// Invoke an AWS Lambda function synchronously and asynchronously.
mod aws_lambda_invoke;

/// A typed message bus: publish Protobuf messages through a trait, with an in-memory implementation
/// for tests and AWS SQS / SNS implementations for production.
mod aws_message_bus;

/// Point the S3 client at an S3-compatible store (MinIO, RustFS, LocalStack), optionally over HTTPS
/// with a private certificate authority.
mod aws_s3_compatible;
/// This example demonstrates how to use the AWS SDK for Rust to list Amazon S3
mod aws_sdk;
/// Send an HTML and plain-text email with Amazon SES v2.
mod aws_ses;

/// Loads and validates the settings that control cloud behavior. Composition code uses this module
/// before starting services so invalid configuration is reported early.
mod aws_shared_config;

/// A minimal Kubernetes operator with `kube-rs`: a custom resource (`Echo`), a pure function that
/// computes the desired `Deployment`, and a controller loop that applies it and reports the status.
mod kubernetes;
/// This example demonstrates a simple web application using the
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
    dispatch(Cli::parse())
}

/// Runs the example selected in the parsed command line.
fn dispatch(cli: Cli) -> Result<()> {
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

/// Tests behavior implemented by the surrounding module.
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
