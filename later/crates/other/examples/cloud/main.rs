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

#[cfg(test)]
mod tests {
    use clap::CommandFactory as _;

    use super::*;

    /// The command-line definition is internally consistent.
    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Without a subcommand, nothing is run.
    #[test]
    fn no_subcommand_runs_nothing() {
        let cli =
            Cli::try_parse_from(["example"]).expect("no arguments are valid");
        assert!(cli.command.is_none());
        dispatch(cli).expect("the example should run successfully");
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
