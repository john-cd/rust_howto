//! Runs one of the `threadpool` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod threadpool_fractal;
mod threadpool_walk;

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
    /// Runs the `threadpool_fractal` example.
    #[command(name = "threadpool_fractal")]
    ThreadpoolFractal,
    /// Runs the `threadpool_walk` example.
    #[command(name = "threadpool_walk")]
    ThreadpoolWalk,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::ThreadpoolFractal => {
                threadpool_fractal::run()?;
            }
            Commands::ThreadpoolWalk => {
                threadpool_walk::run()?;
            }
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
