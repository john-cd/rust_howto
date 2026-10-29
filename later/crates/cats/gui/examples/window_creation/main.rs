//! Runs one of the `window_creation` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "baseview")]
mod baseview;
#[cfg(feature = "softbuffer")]
mod software_pixels;
// Tao's dependency is disabled because its current GTK 3 / glib 0.18 chain is
// incompatible with the workspace. Keep tao.rs and the book example for a
// compatible dependency update.
mod winit;

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
    /// Draws a software pixel buffer in a Winit window.
    #[cfg(feature = "softbuffer")]
    SoftwarePixels,
    /// Runs the `baseview` example.
    #[cfg(feature = "baseview")]
    #[command(name = "baseview")]
    Baseview,
    /// Runs the `winit` example.
    #[command(name = "winit")]
    Winit,
}

/// Runs the example selected on the command line.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "softbuffer")]
            Commands::SoftwarePixels => software_pixels::run()?,
            #[cfg(feature = "baseview")]
            Commands::Baseview => {
                baseview::run()?;
            }
            Commands::Winit => {
                winit::run()?;
            }
        }
    }
    Ok(())
}
