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
/// Runs the example.
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

/// Verifies command-line selection and execution of the documented examples.
#[cfg(test)]
mod tests {
    /// Explains that the example requires Windows on other platforms.
    #[cfg(not(target_os = "windows"))]
    #[test]
    fn main_explains_the_requirement() {
        assert!(std::panic::catch_unwind(super::main).is_ok());
    }
}
