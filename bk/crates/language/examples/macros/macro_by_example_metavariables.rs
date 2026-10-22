//! Metavariables in declarative macros.
//!
//! TODO Reviewed ??-??-2026

#![allow(unused)]

// ANCHOR: example
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

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
