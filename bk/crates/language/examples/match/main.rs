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

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "at_bindings")]
    AtBindings,
    #[command(name = "destructure_enum")]
    DestructureEnum,
    #[command(name = "destructure_reference")]
    DestructureReference,
    #[command(name = "destructure_struct")]
    DestructureStruct,
    #[command(name = "destructure_tuple")]
    DestructureTuple,
    #[command(name = "fn_closure_arguments")]
    FnClosureArguments,
    #[command(name = "for1")]
    For1,
    #[command(name = "if_let")]
    IfLet,
    #[command(name = "ignore_values")]
    IgnoreValues,
    #[command(name = "let1")]
    Let1,
    #[command(name = "let_else")]
    LetElse,
    #[command(name = "literals")]
    Literals,
    #[command(name = "match1")]
    Match1,
    #[command(name = "match2")]
    Match2,
    #[command(name = "match_guards")]
    MatchGuards,
    #[command(name = "or_patterns")]
    OrPatterns,
    #[command(name = "ranges")]
    Ranges,
    #[command(name = "ref_bindings")]
    RefBindings,
    #[command(name = "variable_binding")]
    VariableBinding,
    #[command(name = "while_let")]
    WhileLet,
}

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
