//! Runs one of the `python` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(any(target_os = "linux", target_os = "windows"))]
use clap::Parser;
#[cfg(any(target_os = "linux", target_os = "windows"))]
use clap::Subcommand;

/// Calls embedded Python code and passes it positional arguments with PyO3.
#[cfg(any(target_os = "linux", target_os = "windows"))]
mod pyo3;

/// Demonstrates how to define a PyO3 Python module in Rust, register it with `sys.modules`, and
/// invoke its Rust function and class from embedded Python.
#[cfg(any(target_os = "linux", target_os = "windows"))]
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
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    #[command(name = "use_rust_from_python")]
    UseRustFromPython,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(command) = cli.command {
        match command {
            Commands::Pyo3 => pyo3::run()?,
            Commands::UseRustFromPython => use_rust_from_python::run()?,
        }
    }
    Ok(())
}

/// Parses process arguments and runs the selected Python example.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    {
        dispatch(Cli::parse())
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        Ok(())
    }
}

/// Tests the `main` example and its documented behavior.
#[cfg(all(test, any(target_os = "linux", target_os = "windows")))]
mod tests {
    use clap::CommandFactory;
    use clap::Parser;

    use super::*;

    /// Checks that each Python example is reachable from the CLI.
    #[test]
    fn cli_lists_all_examples() {
        Cli::command().debug_assert();
        for name in ["pyo3", "use_rust_from_python"] {
            let cli = Cli::try_parse_from(["example", name])
                .unwrap_or_else(|error| panic!("`{name}` should parse: {error}"));
            assert!(cli.command.is_some());
        }
    }
}
