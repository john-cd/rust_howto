#![allow(dead_code)]
// ANCHOR: example
macro_rules! accept_single_token_tree {
    ($_:tt) => {}
}

macro_rules! accept_multiple_token_trees {
    ( $( $_:tt )* ) => {}
}

fn main() {
    // A token tree can be a single token, here a string literal.
    accept_single_token_tree!("a");

    // A token tree can be a list of tokens within balanced delimiters.
    // It does not need to be valid Rust.
    accept_single_token_tree!( {"a" ~ async 42} );

    // This macro accepts zero or more token trees (here 6).
    accept_multiple_token_trees!{
        select * from table where ( a > 10 )
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
