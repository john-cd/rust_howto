#![allow(dead_code)]
// ANCHOR: example
macro_rules! accept_single_token_tree {
    ($_:tt) => {};
}

macro_rules! accept_multiple_token_trees {
    ( $( $_:tt )* ) => {};
}

fn main() {
    // A "token tree" can a single lexical token (identifier, Rust keyword,
    // operator, punctuation, or literal):
    accept_single_token_tree!("a");
    accept_single_token_tree!( :: );
    accept_single_token_tree!( , );
    accept_single_token_tree!(for);

    // A token tree can be a list of tokens within balanced delimiters `()`,
    // `[]`, or `{}`:
    accept_single_token_tree!( {"a" ~ async 42} );

    // Use a `$( $_:tt )*` repetition to accept zero or more token trees,
    // here "select", "*", ""from", "table", "where", and "( a > 10 )".
    accept_multiple_token_trees! {
        select * from table where ( a > 10 )
    }
    // Use `$( $name:tt )+` for one or more token trees; `$( $name:tt );*` for
    // semicolon-separated tokens, etc.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
