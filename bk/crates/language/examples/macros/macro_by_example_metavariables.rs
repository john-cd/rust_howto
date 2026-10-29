#![allow(unused)]
// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Metavariables in declarative macros.

/// Prints one, two or more arguments, depending on how many expressions it
/// receives.
macro_rules! print_args {
    // Rule 1: Matches one expression.
    ($e:expr) => {
        println!("Single argument: {:?}", $e);
    };
    // Rule 2: Matches two expressions (separated by a literal comma).
    ($e1:expr, $e2:expr) => {
        println!("Two arguments: {:?} and {:?}", $e1, $e2);
    };
}

/// Calls the macro with different arguments.
fn main() {
    // Note how macros can accept different combinations of
    // arguments by having multiple rules:
    print_args!(10);
    print_args!("hello", true);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the macro by example metavariables example through its entry point.
    #[test]
    fn test_macro_by_example_metavariables_example_entry_point() {
        main();
    }

    // Output-only example: the macro arms only print their arguments.
    /// Both arms of the macro accept expressions.
    #[test]
    fn test_print_args_accepts_expressions() {
        print_args!(1 + 2);
        print_args!(vec![1], Some("x"));
    }
}
