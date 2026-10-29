//! Runs one of the `machine_learning` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

mod forecast;
#[cfg(feature = "onnx")]
mod onnx_infer;
mod smartcore_fit;

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
    /// Runs the `forecast` example.
    #[command(name = "forecast")]
    Forecast,
    /// Runs the `onnx_infer` example.
    #[cfg(feature = "onnx")]
    #[command(name = "onnx_infer")]
    OnnxInfer,
    /// Runs the `smartcore_fit` example.
    #[command(name = "smartcore_fit")]
    SmartcoreFit,
}

/// Runs the example selected on the command line.
fn main() -> Result<()> {
    dispatch(Cli::parse())
}

/// Runs the example selected in the parsed command line.
fn dispatch(cli: Cli) -> Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Forecast => forecast::run()?,
            #[cfg(feature = "onnx")]
            Commands::OnnxInfer => onnx_infer::run()?,
            Commands::SmartcoreFit => smartcore_fit::run()?,
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
