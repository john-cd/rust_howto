//! Runs one of the `window_creation` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod baseview;
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
    /// Runs the `baseview` example.
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
