// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod at_bindings;
mod destructure_enum;
mod destructure_reference;
mod destructure_struct;
mod destructure_tuple;
mod fn_closure_arguments;
mod for1;
mod if_let;
mod ignore_values;
mod let1;
mod let_else;
mod literals;
mod match1;
mod match2;
mod match_guards;
mod or_patterns;
mod ranges;
mod ref_bindings;
mod variable_binding;
mod while_let;

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
    /// Runs the `at_bindings` example.
    #[command(name = "at_bindings")]
    AtBindings,
    /// Runs the `destructure_enum` example.
    #[command(name = "destructure_enum")]
    DestructureEnum,
    /// Runs the `destructure_reference` example.
    #[command(name = "destructure_reference")]
    DestructureReference,
    /// Runs the `destructure_struct` example.
    #[command(name = "destructure_struct")]
    DestructureStruct,
    /// Runs the `destructure_tuple` example.
    #[command(name = "destructure_tuple")]
    DestructureTuple,
    /// Runs the `fn_closure_arguments` example.
    #[command(name = "fn_closure_arguments")]
    FnClosureArguments,
    /// Runs the `for1` example.
    #[command(name = "for1")]
    For1,
    /// Runs the `if_let` example.
    #[command(name = "if_let")]
    IfLet,
    /// Runs the `ignore_values` example.
    #[command(name = "ignore_values")]
    IgnoreValues,
    /// Runs the `let1` example.
    #[command(name = "let1")]
    Let1,
    /// Runs the `let_else` example.
    #[command(name = "let_else")]
    LetElse,
    /// Runs the `literals` example.
    #[command(name = "literals")]
    Literals,
    /// Runs the `match1` example.
    #[command(name = "match1")]
    Match1,
    /// Runs the `match2` example.
    #[command(name = "match2")]
    Match2,
    /// Runs the `match_guards` example.
    #[command(name = "match_guards")]
    MatchGuards,
    /// Runs the `or_patterns` example.
    #[command(name = "or_patterns")]
    OrPatterns,
    /// Runs the `ranges` example.
    #[command(name = "ranges")]
    Ranges,
    /// Runs the `ref_bindings` example.
    #[command(name = "ref_bindings")]
    RefBindings,
    /// Runs the `variable_binding` example.
    #[command(name = "variable_binding")]
    VariableBinding,
    /// Runs the `while_let` example.
    #[command(name = "while_let")]
    WhileLet,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AtBindings => {
                at_bindings::run();
            }
            Commands::DestructureEnum => {
                destructure_enum::run();
            }
            Commands::DestructureReference => {
                destructure_reference::run();
            }
            Commands::DestructureStruct => {
                destructure_struct::run();
            }
            Commands::DestructureTuple => {
                destructure_tuple::run();
            }
            Commands::FnClosureArguments => {
                fn_closure_arguments::run();
            }
            Commands::For1 => {
                for1::run();
            }
            Commands::IfLet => {
                if_let::run();
            }
            Commands::IgnoreValues => {
                ignore_values::run();
            }
            Commands::Let1 => {
                let1::run();
            }
            Commands::LetElse => {
                let_else::run();
            }
            Commands::Literals => {
                literals::run();
            }
            Commands::Match1 => {
                match1::run();
            }
            Commands::Match2 => {
                match2::run();
            }
            Commands::MatchGuards => {
                match_guards::run();
            }
            Commands::OrPatterns => {
                or_patterns::run();
            }
            Commands::Ranges => {
                ranges::run();
            }
            Commands::RefBindings => {
                ref_bindings::run();
            }
            Commands::VariableBinding => {
                variable_binding::run();
            }
            Commands::WhileLet => {
                while_let::run();
            }
        }
    }
}
