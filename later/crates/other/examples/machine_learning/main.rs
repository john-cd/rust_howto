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
    let cli = Cli::parse();

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
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
