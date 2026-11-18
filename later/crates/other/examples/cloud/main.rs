//! Runs one of the `cloud` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

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
    /// Prints the CRD or starts the controller against an explicit kubeconfig.
    #[command(name = "kubernetes")]
    Kubernetes {
        /// Starts the controller with this kubeconfig file.
        #[arg(long, value_name = "FILE")]
        kubeconfig: Option<PathBuf>,
    },

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
            Commands::Kubernetes { kubeconfig } => kubernetes::run(kubeconfig.as_deref())?,
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
