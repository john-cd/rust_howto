#![allow(unused)]
// ANCHOR: example
/// A basic macro-by-example (from the Rust By Example book).
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!")
    };
}

fn main() {
    // This call will expand into `println!("Hello!")`
    say_hello!();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
