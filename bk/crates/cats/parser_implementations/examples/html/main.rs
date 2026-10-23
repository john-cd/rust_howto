//! Runs one of the `html` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod cssparser;
mod html5ever;
mod tl;

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
    /// Runs the `cssparser` example.
    #[command(name = "cssparser")]
    Cssparser,
    /// Runs the `html5ever` example.
    #[command(name = "html5ever")]
    Html5ever,
    /// Runs the `tl` example.
    #[command(name = "tl")]
    Tl,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Cssparser => {
                cssparser::run();
            }
            Commands::Html5ever => {
                html5ever::run();
            }
            Commands::Tl => {
                tl::run()?;
            }
        }
    }
    Ok(())
}
