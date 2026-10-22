// TODO Reviewed ??-??-2026
#[cfg(target_os = "linux")]
use clap::Parser;
#[cfg(target_os = "linux")]
use clap::Subcommand;

#[cfg(target_os = "linux")]
mod pyo3;
#[cfg(target_os = "linux")]
mod use_rust_from_python;

/// Command-line arguments.
#[cfg(target_os = "linux")]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(target_os = "linux")]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `pyo3` example.
    #[cfg(target_os = "linux")]
    #[command(name = "pyo3")]
    Pyo3,
    /// Runs the `use_rust_from_python` example.
    #[cfg(target_os = "linux")]
    #[command(name = "use_rust_from_python")]
    UseRustFromPython,
}

/// Runs the example selected on the command line.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(target_os = "linux")]
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
