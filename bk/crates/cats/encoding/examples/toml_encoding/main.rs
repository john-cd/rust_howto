// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod toml;
mod toml1;

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
    /// Runs the `toml` example.
    #[command(name = "toml")]
    Toml,
    /// Runs the `toml1` example.
    #[command(name = "toml1")]
    Toml1,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Toml => {
                toml::run()?;
            }
            Commands::Toml1 => {
                toml1::run()?;
            }
        }
    }
    Ok(())
}
