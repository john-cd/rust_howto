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

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "macro_by_example_basic")]
    MacroByExampleBasic,
    #[command(name = "macro_by_example_dsl")]
    MacroByExampleDsl,
    #[command(name = "macro_by_example_hygiene")]
    MacroByExampleHygiene,
    #[command(name = "macro_by_example_metavariables")]
    MacroByExampleMetavariables,
    #[command(name = "macro_by_example_repetitions")]
    MacroByExampleRepetitions,
    #[command(name = "macro_by_example_type_checking")]
    MacroByExampleTypeChecking,
    #[command(name = "macro_by_example_type_checking2")]
    MacroByExampleTypeChecking2,
    #[command(name = "macros")]
    Macros,
    #[command(name = "proc_macro2_example")]
    ProcMacro2Example,
    #[command(name = "proc_macro_attribute")]
    ProcMacroAttribute,
    #[command(name = "proc_macro_derive")]
    ProcMacroDerive,
    #[command(name = "proc_macro_function")]
    ProcMacroFunction,
    #[command(name = "token_tree")]
    TokenTree,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::MacroByExampleBasic => {
                let _ = macro_by_example_basic::run();
            }
            Commands::MacroByExampleDsl => {
                let _ = macro_by_example_dsl::run();
            }
            Commands::MacroByExampleHygiene => {
                let _ = macro_by_example_hygiene::run();
            }
            Commands::MacroByExampleMetavariables => {
                let _ = macro_by_example_metavariables::run();
            }
            Commands::MacroByExampleRepetitions => {
                let _ = macro_by_example_repetitions::run();
            }
            Commands::MacroByExampleTypeChecking => {
                let _ = macro_by_example_type_checking::run();
            }
            Commands::MacroByExampleTypeChecking2 => {
                let _ = macro_by_example_type_checking2::run();
            }
            Commands::Macros => {
                let _ = macros::run();
            }
            Commands::ProcMacro2Example => {
                let _ = proc_macro2_example::run();
            }
            Commands::ProcMacroAttribute => {
                let _ = proc_macro_attribute::run();
            }
            Commands::ProcMacroDerive => {
                let _ = proc_macro_derive::run();
            }
            Commands::ProcMacroFunction => {
                let _ = proc_macro_function::run();
            }
            Commands::TokenTree => {
                let _ = token_tree::run();
            }
        }
    }
}
