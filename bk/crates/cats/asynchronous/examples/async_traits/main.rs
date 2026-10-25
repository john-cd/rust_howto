//! Runs one of the `async_traits` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod afit;
mod async_trait_dyn;
mod async_traits;
mod async_traits2;
mod async_traits3;
mod rpitit_send;

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
    /// Runs the `async_traits` example.
    #[command(name = "async_traits")]
    AsyncTraits,
    /// Runs the `async_traits2` example.
    #[command(name = "async_traits2")]
    AsyncTraits2,
    /// Runs the `async_traits3` example.
    #[command(name = "async_traits3")]
    AsyncTraits3,
    /// Runs the `afit` example.
    #[command(name = "afit")]
    Afit,
    /// Runs the `rpitit_send` example.
    #[command(name = "rpitit_send")]
    RpititSend,
    /// Runs the `async_trait_dyn` example.
    #[command(name = "async_trait_dyn")]
    AsyncTraitDyn,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AsyncTraits => {
                async_traits::run();
            }
            Commands::AsyncTraits2 => {
                async_traits2::run();
            }
            Commands::AsyncTraits3 => {
                async_traits3::run();
            }
            Commands::Afit => {
                afit::run();
            }
            Commands::RpititSend => {
                rpitit_send::run();
            }
            Commands::AsyncTraitDyn => {
                async_trait_dyn::run();
            }
        }
    }
}
