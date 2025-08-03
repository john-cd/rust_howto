#![allow(unused)]
// ANCHOR: example
/// A basic macro-by-example (from the Rust By Example book).
macro_rules! say_hello {
    // The matcher `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        let w = "World";
        println!("Hello {w}!");
    };
}

fn main() {
    say_hello!();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
