// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod result;
mod result2;
mod result3;

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
    /// Runs the `result` example.
    #[command(name = "result")]
    Result,
    /// Runs the `result2` example.
    #[command(name = "result2")]
    Result2,
    /// Runs the `result3` example.
    #[command(name = "result3")]
    Result3,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Result => {
                result::run().map_err(|e| anyhow::anyhow!("{e}"))?;
            }
            Commands::Result2 => {
                result2::run();
            }
            Commands::Result3 => {
                result3::run();
            }
        }
    }
    Ok(())
}
