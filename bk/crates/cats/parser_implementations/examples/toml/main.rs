//! Runs one of the `toml` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod basic_toml;
mod toml;
mod toml_edit;

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
    /// Runs the `basic_toml` example.
    #[command(name = "basic_toml")]
    BasicToml,
    /// Runs the `toml` example.
    #[command(name = "toml")]
    Toml,
    /// Runs the `toml_edit` example.
    #[command(name = "toml_edit")]
    TomlEdit,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::BasicToml => {
                basic_toml::run()?;
            }
            Commands::Toml => {
                toml::run();
            }
            Commands::TomlEdit => {
                toml_edit::run()?;
            }
        }
    }
    Ok(())
}
