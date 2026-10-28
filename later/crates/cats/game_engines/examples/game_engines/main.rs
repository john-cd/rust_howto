//! Runs one of the `game_engines` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "bevy")]
mod bevy;
#[cfg(feature = "fyrox")]
mod fyrox;
#[cfg(feature = "ggez")]
mod ggez;
mod macroquad;

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
    /// Runs the `bevy` example.
    #[command(name = "bevy")]
    #[cfg(feature = "bevy")]
    Bevy,
    /// Runs the `fyrox` example.
    #[command(name = "fyrox")]
    #[cfg(feature = "fyrox")]
    Fyrox,
    /// Runs the `ggez` example.
    #[command(name = "ggez")]
    #[cfg(feature = "ggez")]
    Ggez,
    /// Runs the `macroquad` example.
    #[command(name = "macroquad")]
    Macroquad,
}

/// Runs the example selected on the command line.
///
/// # Errors
///
/// Returns an error if the selected engine's setup or event loop fails.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "bevy")]
            Commands::Bevy => {
                bevy::run();
            }
            #[cfg(feature = "fyrox")]
            Commands::Fyrox => {
                fyrox::run();
            }
            #[cfg(feature = "ggez")]
            Commands::Ggez => {
                ggez::run()?;
            }
            Commands::Macroquad => {
                macroquad::run();
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
