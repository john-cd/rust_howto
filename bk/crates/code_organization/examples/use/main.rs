//! Runs one of the `use` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod pub_use;
mod use1;
mod use2;
mod use3;
mod use_external_crate;
mod use_shortcuts;

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
    /// Runs the `pub_use` example.
    #[command(name = "pub_use")]
    PubUse,
    /// Runs the `use1` example.
    #[command(name = "use1")]
    Use1,
    /// Runs the `use2` example.
    #[command(name = "use2")]
    Use2,
    /// Runs the `use3` example.
    #[command(name = "use3")]
    Use3,
    /// Runs the `use_external_crate` example.
    #[command(name = "use_external_crate")]
    UseExternalCrate,
    /// Runs the `use_shortcuts` example.
    #[command(name = "use_shortcuts")]
    UseShortcuts,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::PubUse => {
                pub_use::run();
            }
            Commands::Use1 => {
                use1::run();
            }
            Commands::Use2 => {
                use2::run();
            }
            Commands::Use3 => {
                use3::run();
            }
            Commands::UseExternalCrate => {
                use_external_crate::run()?;
            }
            Commands::UseShortcuts => {
                use_shortcuts::run();
            }
        }
    }
    Ok(())
}
