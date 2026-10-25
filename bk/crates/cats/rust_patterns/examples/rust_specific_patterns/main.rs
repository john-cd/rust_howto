//! Runs one of the `rust_specific_patterns` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod dyn_clone;
mod newtype_equality;
mod newtype_units;
mod pin_project;
mod validated_newtype;

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
    /// Runs the `dyn_clone` example.
    #[command(name = "dyn_clone")]
    DynClone,
    /// Runs the `pin_project` example.
    #[command(name = "pin_project")]
    PinProject,
    /// Runs the `newtype_equality` example.
    #[command(name = "newtype_equality")]
    NewtypeEquality,
    /// Runs the `newtype_units` example.
    #[command(name = "newtype_units")]
    NewtypeUnits,
    /// Runs the `validated_newtype` example.
    #[command(name = "validated_newtype")]
    ValidatedNewtype,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::DynClone => {
                dyn_clone::run();
            }
            Commands::PinProject => {
                pin_project::run();
            }
            Commands::NewtypeEquality => {
                newtype_equality::run();
            }
            Commands::NewtypeUnits => {
                newtype_units::run();
            }
            Commands::ValidatedNewtype => {
                validated_newtype::run();
            }
        }
    }
}
