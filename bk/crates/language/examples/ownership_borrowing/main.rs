//! Runs one of the `ownership_borrowing` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod borrowing_function;
mod borrowing_function_mutable;
mod borrowing_immutable;
mod clone;
mod copy;
mod move1;
mod move_function;
mod read_write_lock;
mod scope;

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
    /// Runs the `borrowing_function` example.
    #[command(name = "borrowing_function")]
    BorrowingFunction,
    /// Runs the `borrowing_function_mutable` example.
    #[command(name = "borrowing_function_mutable")]
    BorrowingFunctionMutable,
    /// Runs the `borrowing_immutable` example.
    #[command(name = "borrowing_immutable")]
    BorrowingImmutable,
    /// Runs the `clone` example.
    #[command(name = "clone")]
    Clone,
    /// Runs the `copy` example.
    #[command(name = "copy")]
    Copy,
    /// Runs the `move1` example.
    #[command(name = "move1")]
    Move1,
    /// Runs the `move_function` example.
    #[command(name = "move_function")]
    MoveFunction,
    /// Runs the `read_write_lock` example.
    #[command(name = "read_write_lock")]
    ReadWriteLock,
    /// Runs the `scope` example.
    #[command(name = "scope")]
    Scope,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::BorrowingFunction => {
                borrowing_function::run();
            }
            Commands::BorrowingFunctionMutable => {
                borrowing_function_mutable::run();
            }
            Commands::BorrowingImmutable => {
                borrowing_immutable::run();
            }
            Commands::Clone => {
                clone::run();
            }
            Commands::Copy => {
                copy::run();
            }
            Commands::Move1 => {
                move1::run();
            }
            Commands::MoveFunction => {
                move_function::run();
            }
            Commands::ReadWriteLock => {
                read_write_lock::run();
            }
            Commands::Scope => {
                scope::run();
            }
        }
    }
}
