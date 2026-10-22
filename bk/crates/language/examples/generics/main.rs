//! Runs one of the `generics` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod const_generics;
mod const_generics2;
mod generic_lifetime;
mod generic_type_parameter;

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
    /// Runs the `const_generics` example.
    #[command(name = "const_generics")]
    ConstGenerics,
    /// Runs the `const_generics2` example.
    #[command(name = "const_generics2")]
    ConstGenerics2,
    /// Runs the `generic_lifetime` example.
    #[command(name = "generic_lifetime")]
    GenericLifetime,
    /// Runs the `generic_type_parameter` example.
    #[command(name = "generic_type_parameter")]
    GenericTypeParameter,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::ConstGenerics => {
                const_generics::run();
            }
            Commands::ConstGenerics2 => {
                const_generics2::run();
            }
            Commands::GenericLifetime => {
                generic_lifetime::run();
            }
            Commands::GenericTypeParameter => {
                generic_type_parameter::run();
            }
        }
    }
}
