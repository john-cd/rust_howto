// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod diff;
mod similar;

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
    /// Runs the `diff` example.
    #[command(name = "diff")]
    Diff,
    /// Runs the `similar` example.
    #[command(name = "similar")]
    Similar,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Diff => {
                diff::run()?;
            }
            Commands::Similar => {
                similar::run();
            }
        }
    }
    Ok(())
}
