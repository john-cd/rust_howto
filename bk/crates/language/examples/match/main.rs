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
                let _ = at_bindings::run();
            }
            Commands::DestructureEnum => {
                let _ = destructure_enum::run();
            }
            Commands::DestructureReference => {
                let _ = destructure_reference::run();
            }
            Commands::DestructureStruct => {
                let _ = destructure_struct::run();
            }
            Commands::DestructureTuple => {
                let _ = destructure_tuple::run();
            }
            Commands::FnClosureArguments => {
                let _ = fn_closure_arguments::run();
            }
            Commands::For1 => {
                let _ = for1::run();
            }
            Commands::IfLet => {
                let _ = if_let::run();
            }
            Commands::IgnoreValues => {
                let _ = ignore_values::run();
            }
            Commands::Let1 => {
                let _ = let1::run();
            }
            Commands::LetElse => {
                let _ = let_else::run();
            }
            Commands::Literals => {
                let _ = literals::run();
            }
            Commands::Match1 => {
                let _ = match1::run();
            }
            Commands::Match2 => {
                let _ = match2::run();
            }
            Commands::MatchGuards => {
                let _ = match_guards::run();
            }
            Commands::OrPatterns => {
                let _ = or_patterns::run();
            }
            Commands::Ranges => {
                let _ = ranges::run();
            }
            Commands::RefBindings => {
                let _ = ref_bindings::run();
            }
            Commands::VariableBinding => {
                let _ = variable_binding::run();
            }
            Commands::WhileLet => {
                let _ = while_let::run();
            }
        }
    }
}
