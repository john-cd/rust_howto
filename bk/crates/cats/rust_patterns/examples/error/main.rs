//! Runs one of the `error` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod anyhow;
mod backtrace;
mod color_eyre;
mod error_handling;
mod miette;
mod panic;
mod question_mark;
mod question_mark2;
mod retain;
mod thiserror;
mod thiserror2;
mod unwrap;
mod unwrap_or_else;

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
    /// Runs the `anyhow` example.
    #[command(name = "anyhow")]
    Anyhow,
    /// Runs the `backtrace` example.
    #[command(name = "backtrace")]
    Backtrace,
    /// Runs the `color_eyre` example.
    #[command(name = "color_eyre")]
    ColorEyre,
    /// Runs the `error_handling` example.
    #[command(name = "error_handling")]
    ErrorHandling,
    /// Runs the `miette` example.
    #[command(name = "miette")]
    Miette,
    /// Runs the `panic` example.
    #[command(name = "panic")]
    Panic,
    /// Runs the `question_mark` example.
    #[command(name = "question_mark")]
    QuestionMark,
    /// Runs the `question_mark2` example.
    #[command(name = "question_mark2")]
    QuestionMark2,
    /// Runs the `retain` example.
    #[command(name = "retain")]
    Retain,
    /// Runs the `thiserror` example.
    #[command(name = "thiserror")]
    Thiserror,
    /// Runs the `thiserror2` example.
    #[command(name = "thiserror2")]
    Thiserror2,
    /// Runs the `unwrap` example.
    #[command(name = "unwrap")]
    Unwrap,
    /// Runs the `unwrap_or_else` example.
    #[command(name = "unwrap_or_else")]
    UnwrapOrElse,
}

/// Runs the example selected on the command line.
fn main() -> ::anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Anyhow => {
                anyhow::run()?;
            }
            Commands::Backtrace => {
                backtrace::run()
                    .map_err(|e| ::anyhow::anyhow!(e.to_string()))?;
            }
            Commands::ColorEyre => {
                color_eyre::run()
                    .map_err(|e| ::anyhow::anyhow!(e.to_string()))?;
            }
            Commands::ErrorHandling => {
                error_handling::run();
            }
            Commands::Miette => {
                miette::run().map_err(|e| ::anyhow::anyhow!(e.to_string()))?;
            }
            Commands::Panic => {
                panic::run();
            }
            Commands::QuestionMark => {
                question_mark::run();
            }
            Commands::QuestionMark2 => {
                question_mark2::run();
            }
            Commands::Retain => {
                retain::run()?;
            }
            Commands::Thiserror => {
                thiserror::run()
                    .map_err(|e| ::anyhow::anyhow!(e.to_string()))?;
            }
            Commands::Thiserror2 => {
                thiserror2::run();
            }
            Commands::Unwrap => {
                unwrap::run();
            }
            Commands::UnwrapOrElse => {
                unwrap_or_else::run();
            }
        }
    }

    Ok(())
}
