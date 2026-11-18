//! Runs one of the `windows` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(target_os = "windows")]
use clap::Parser;
#[cfg(target_os = "windows")]
use clap::Subcommand;

/// Displays a message box through the raw `winapi` bindings.
#[cfg(target_os = "windows")]
mod winapi;

/// This example demonstrates how to use the `windows` crate to call the `MessageBoxW` function from
/// the Windows API.
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
///
/// # Errors
///
/// Returns the Windows error if the message box cannot be displayed.
#[cfg(target_os = "windows")]
fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Winapi => {
                winapi::run()?;
            }
            Commands::Windows => {
                windows::run()?;
            }
        }
    }
    Ok(())
}

/// Explains that this example requires Windows when built on another platform.
#[cfg(not(target_os = "windows"))]
fn main() {
    println!("This example requires Windows.");
}
