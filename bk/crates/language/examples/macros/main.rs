//! Runs one of the `macros` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod macro_by_example_basic;
mod macro_by_example_dsl;
mod macro_by_example_hygiene;
mod macro_by_example_metavariables;
mod macro_by_example_repetitions;
mod macro_by_example_type_checking;
mod macro_by_example_type_checking2;
mod macros;
mod proc_macro2_example;
mod proc_macro_attribute;
mod proc_macro_derive;
mod proc_macro_function;
mod token_tree;

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
    /// Runs the `macro_by_example_basic` example.
    #[command(name = "macro_by_example_basic")]
    MacroByExampleBasic,
    /// Runs the `macro_by_example_dsl` example.
    #[command(name = "macro_by_example_dsl")]
    MacroByExampleDsl,
    /// Runs the `macro_by_example_hygiene` example.
    #[command(name = "macro_by_example_hygiene")]
    MacroByExampleHygiene,
    /// Runs the `macro_by_example_metavariables` example.
    #[command(name = "macro_by_example_metavariables")]
    MacroByExampleMetavariables,
    /// Runs the `macro_by_example_repetitions` example.
    #[command(name = "macro_by_example_repetitions")]
    MacroByExampleRepetitions,
    /// Runs the `macro_by_example_type_checking` example.
    #[command(name = "macro_by_example_type_checking")]
    MacroByExampleTypeChecking,
    /// Runs the `macro_by_example_type_checking2` example.
    #[command(name = "macro_by_example_type_checking2")]
    MacroByExampleTypeChecking2,
    /// Runs the `macros` example.
    #[command(name = "macros")]
    Macros,
    /// Runs the `proc_macro2_example` example.
    #[command(name = "proc_macro2_example")]
    ProcMacro2Example,
    /// Runs the `proc_macro_attribute` example.
    #[command(name = "proc_macro_attribute")]
    ProcMacroAttribute,
    /// Runs the `proc_macro_derive` example.
    #[command(name = "proc_macro_derive")]
    ProcMacroDerive,
    /// Runs the `proc_macro_function` example.
    #[command(name = "proc_macro_function")]
    ProcMacroFunction,
    /// Runs the `token_tree` example.
    #[command(name = "token_tree")]
    TokenTree,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::MacroByExampleBasic => {
                macro_by_example_basic::run();
            }
            Commands::MacroByExampleDsl => {
                macro_by_example_dsl::run();
            }
            Commands::MacroByExampleHygiene => {
                macro_by_example_hygiene::run();
            }
            Commands::MacroByExampleMetavariables => {
                macro_by_example_metavariables::run();
            }
            Commands::MacroByExampleRepetitions => {
                macro_by_example_repetitions::run();
            }
            Commands::MacroByExampleTypeChecking => {
                macro_by_example_type_checking::run();
            }
            Commands::MacroByExampleTypeChecking2 => {
                macro_by_example_type_checking2::run();
            }
            Commands::Macros => {
                macros::run();
            }
            Commands::ProcMacro2Example => {
                proc_macro2_example::run();
            }
            Commands::ProcMacroAttribute => {
                proc_macro_attribute::run();
            }
            Commands::ProcMacroDerive => {
                proc_macro_derive::run();
            }
            Commands::ProcMacroFunction => {
                proc_macro_function::run();
            }
            Commands::TokenTree => {
                token_tree::run();
            }
        }
    }
}
