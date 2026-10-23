#![allow(unused)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A basic declarative macro.

/// A basic macro-by-example (from the Rust By Example book).
macro_rules! say_hello {
    // The matcher `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        let w = "World";
        println!("Hello {w}!");
    };
}

/// Calls the macro.
fn main() {
    say_hello!();
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
