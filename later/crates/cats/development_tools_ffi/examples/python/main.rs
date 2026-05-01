#[cfg(target_os = "linux")]
use clap::{Parser, Subcommand};

#[cfg(target_os = "linux")]
mod pyo3;
#[cfg(target_os = "linux")]
mod use_rust_from_python;

#[cfg(target_os = "linux")]
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[cfg(target_os = "linux")]
#[derive(Subcommand)]
enum Commands {
    #[cfg(target_os = "linux")]
    #[command(name = "pyo3")]
    Pyo3,
    #[cfg(target_os = "linux")]
    #[command(name = "use_rust_from_python")]
    UseRustFromPython,
}

fn main() {
    #[cfg(target_os = "linux")]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(target_os = "linux")]
                Commands::Pyo3 => {
                    let _ = pyo3::run();
                }
                #[cfg(target_os = "linux")]
                Commands::UseRustFromPython => {
                    let _ = use_rust_from_python::run();
                }
            }
        }
    }
}
