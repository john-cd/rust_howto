//! Runs one of the `match` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Binding a value to a name while matching a pattern with `@`.
mod at_bindings;
/// Destructuring an enum in a `match`.
mod destructure_enum;
/// Destructuring references.
mod destructure_reference;
/// Destructuring a struct into its fields.
mod destructure_struct;
/// Destructuring a tuple in a `match`.
mod destructure_tuple;
/// Destructuring in function and closure arguments.
mod fn_closure_arguments;
/// Destructuring in a `for` loop.
mod for1;
/// The `if let` syntax allows you to combine `if` and `let` to handle values that match a specific
/// pattern, while ignoring others.
mod if_let;
/// Ignoring values in patterns with `_` and `..`.
mod ignore_values;
/// Destructuring in a `let` statement.
mod let1;
/// The `let ...
mod let_else;
/// Matching literals.
mod literals;
/// This example defines different types of US coins using an `enum` and then uses a `match`
/// expression to determine the value of a given coin in cents, demonstrating how `match` can handle
/// different `enum` variants and extract data associated with them.
mod match1;
/// Matching a struct against patterns.
mod match2;
/// Match guards, the `if` conditions of match arms.
mod match_guards;
/// Matching one of several patterns with `|`.
mod or_patterns;
/// Matching ranges of values.
mod ranges;
/// Variable binding moves or copies the bound value by default.
mod ref_bindings;
/// Binding the content of an `Option` to a variable with `if let`.
mod variable_binding;
/// Looping with `while let`.
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
fn dispatch(cli: Cli) {
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
/// Runs the example.
/// Parses the command line and runs the selected example.
fn main() {
    dispatch(Cli::parse());
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &[
        "at_bindings",
        "destructure_enum",
        "destructure_reference",
        "destructure_struct",
        "destructure_tuple",
        "fn_closure_arguments",
        "for1",
        "if_let",
        "ignore_values",
        "let1",
        "let_else",
        "literals",
        "match1",
        "match2",
        "match_guards",
        "or_patterns",
        "ranges",
        "ref_bindings",
        "variable_binding",
        "while_let",
    ];

    #[test]
    fn test_cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Each example is reachable through its own subcommand.
    #[test]
    fn test_every_subcommand_parses() {
        for name in SUBCOMMANDS {
            let cli = Cli::try_parse_from(["example", name])
                .unwrap_or_else(|e| panic!("`{name}` should parse: {e}"));
            assert!(cli.command.is_some(), "`{name}` selected no command");
        }
    }

    /// Subcommand names are unique.
    #[test]
    fn test_subcommand_names_are_unique() {
        let mut sorted = SUBCOMMANDS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), SUBCOMMANDS.len());
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }

    /// Running without a subcommand selects nothing and succeeds.
    #[test]
    fn test_no_subcommand_runs_nothing() {
        let cli = Cli::try_parse_from(["example"]).expect("no subcommand is accepted");
        assert!(cli.command.is_none());
        dispatch(cli);
    }
}
