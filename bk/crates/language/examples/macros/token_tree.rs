#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Token trees as the arguments of declarative macros.

/// Accepts a single token tree and expands to nothing.
macro_rules! accept_single_token_tree {
    ($_:tt) => {};
}

/// Accepts any number of token trees and expands to nothing.
macro_rules! accept_multiple_token_trees {
    ( $( $_:tt )* ) => {};
}

/// Calls macros with single tokens and groups of tokens.
fn main() {
    // A "token tree" can be a single lexical token (identifier, Rust keyword,
    // operator, punctuation, or literal):
    accept_single_token_tree!("a");
    accept_single_token_tree!( :: );
    accept_single_token_tree!( , );
    accept_single_token_tree!(for);

    // A token tree can be a list of tokens within balanced delimiters `()`,
    // `[]`, or `{}`:
    accept_single_token_tree!( {"a" ~ async 42} );

    // Use a `$( $_:tt )*` repetition to accept zero or more token trees,
    // here "select", "*", "from", "table", "where", and "( a > 10 )".
    accept_multiple_token_trees! {
        select * from table where ( a > 10 )
    }
    // Use `$( $name:tt )+` for one or more token trees; `$( $name:tt );*` for
    // semicolon-separated tokens, etc.
    println!("Token tree examples complete");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the token tree example through its entry point.
    #[test]
    fn test_token_tree_example_entry_point() {
        main();
    }
}
