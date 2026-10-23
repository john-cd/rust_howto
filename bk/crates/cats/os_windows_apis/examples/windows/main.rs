//! Runs one of the `windows` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(target_os = "windows")]
use clap::Parser;
#[cfg(target_os = "windows")]
use clap::Subcommand;

#[cfg(target_os = "windows")]
mod winapi;
#[cfg(target_os = "windows")]
mod windows;

/// Command-line arguments.
#[cfg(target_os = "windows")]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(target_os = "windows")]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `winapi` example.
    #[command(name = "winapi")]
    Winapi,
    /// Runs the `windows` example.
    #[command(name = "windows")]
    Windows,
}

/// Runs the example selected on the command line.
#[cfg(target_os = "windows")]
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Winapi => {
                winapi::run();
            }
            Commands::Windows => {
                windows::run();
            }
        }
    }
}

/// Explains that this example requires Windows when built on another platform.
#[cfg(not(target_os = "windows"))]
fn main() {
    println!("This example requires Windows.");
}
