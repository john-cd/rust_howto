//! Runs one of the `2d_renderers` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "femtovg")]
mod femtovg;
#[cfg(feature = "minifb")]
mod minifb;
#[cfg(feature = "skia")]
mod skia_safe;
// #[cfg(all(feature = "vello", not(windows)))]
// // TODO review Vello support for Windows
// mod vello;
#[cfg(feature = "vger")]
mod vger;
#[cfg(all(feature = "webrender", not(windows)))]
// TODO Webrender support Windows
mod webrender;

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
    /// Runs the `minifb` example.
    #[cfg(feature = "minifb")]
    #[command(name = "minifb")]
    Minifb,
    /// Runs the `skia_safe` example.
    #[cfg(feature = "skia")]
    #[command(name = "skia_safe")]
    SkiaSafe,
    // #[cfg(all(feature = "vello", not(windows)))]
    // #[command(name = "vello")]
    // Vello,
    /// Runs the `vger` example.
    #[cfg(feature = "vger")]
    #[command(name = "vger")]
    Vger,
    /// Runs the `webrender` example.
    #[cfg(all(feature = "webrender", not(windows)))]
    #[command(name = "webrender")]
    Webrender,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "femtovg")]
            Commands::Femtovg => {
                femtovg::run();
            }
            #[cfg(feature = "minifb")]
            Commands::Minifb => {
                minifb::run()?;
            }
            #[cfg(feature = "skia")]
            Commands::SkiaSafe => {
                skia_safe::run();
            }
            // #[cfg(all(feature = "vello", not(windows)))]
            // Commands::Vello => {
            //     vello::run();
            // }
            #[cfg(feature = "vger")]
            Commands::Vger => {
                vger::run();
            }
            #[cfg(all(feature = "webrender", not(windows)))]
            Commands::Webrender => {
                webrender::run();
            }
        }
    }

    Ok(())
}
