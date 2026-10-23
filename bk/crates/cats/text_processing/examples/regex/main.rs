//! Runs one of the `regex` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod email;
mod fancy_regex;
mod filter_log;
mod hashtags;
mod phone;
mod regex;
mod replace;

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
    /// Runs the `email` example.
    #[command(name = "email")]
    Email,
    /// Runs the `fancy_regex` example.
    #[command(name = "fancy_regex")]
    FancyRegex,
    /// Runs the `filter_log` example.
    #[command(name = "filter_log")]
    FilterLog,
    /// Runs the `hashtags` example.
    #[command(name = "hashtags")]
    Hashtags,
    /// Runs the `phone` example.
    #[command(name = "phone")]
    Phone,
    /// Runs the `regex` example.
    #[command(name = "regex")]
    Regex,
    /// Runs the `replace` example.
    #[command(name = "replace")]
    Replace,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Email => {
                email::run();
            }
            Commands::FancyRegex => {
                fancy_regex::run();
            }
            Commands::FilterLog => {
                filter_log::run()?;
            }
            Commands::Hashtags => {
                hashtags::run();
            }
            Commands::Phone => {
                phone::run()?;
            }
            Commands::Regex => {
                regex::run();
            }
            Commands::Replace => {
                replace::run();
            }
        }
    }
    Ok(())
}
