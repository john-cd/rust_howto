//! Runs one of the `2d_renderers` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "femtovg")]
mod femtovg;
#[cfg(feature = "skia")]
mod skia_safe;
// #[cfg(all(feature = "vello", not(windows)))]
// // TODO review Vello support for Windows
// mod vello;

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
    /// Runs the `femtovg` example.
    #[cfg(feature = "femtovg")]
    #[command(name = "femtovg")]
    Femtovg,
    /// Runs the `skia_safe` example.
    #[cfg(feature = "skia")]
    #[command(name = "skia_safe")]
    SkiaSafe,
    // #[cfg(all(feature = "vello", not(windows)))]
    // #[command(name = "vello")]
    // Vello,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "femtovg")]
            Commands::Femtovg => {
                femtovg::run()?;
            }
            #[cfg(feature = "skia")]
            Commands::SkiaSafe => {
                skia_safe::run();
            }
            // #[cfg(all(feature = "vello", not(windows)))]
            // Commands::Vello => {
            //     vello::run();
            // }
        }
    }

    Ok(())
}
