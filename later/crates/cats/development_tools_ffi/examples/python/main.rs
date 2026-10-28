//! Runs one of the `python` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(any(target_os = "linux", target_os = "windows"))]
use clap::Parser;
#[cfg(any(target_os = "linux", target_os = "windows"))]
use clap::Subcommand;

#[cfg(any(target_os = "linux", target_os = "windows"))]
mod pyo3;
#[cfg(target_os = "linux")]
mod use_rust_from_python;

/// Command-line arguments.
#[cfg(any(target_os = "linux", target_os = "windows"))]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(any(target_os = "linux", target_os = "windows"))]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `pyo3` example.
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    #[command(name = "pyo3")]
    Pyo3,
    /// Runs the `use_rust_from_python` example.
    #[cfg(target_os = "linux")]
    #[command(name = "use_rust_from_python")]
    UseRustFromPython,
}

/// Runs the example selected on the command line.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(any(target_os = "linux", target_os = "windows"))]
                Commands::Pyo3 => {
                    pyo3::run()?;
                }
                #[cfg(target_os = "linux")]
                Commands::UseRustFromPython => {
                    use_rust_from_python::run()?;
                }
            }
        }
    }
    Ok(())
}
