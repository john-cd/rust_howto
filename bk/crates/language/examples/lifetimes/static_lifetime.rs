#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! The `'static` lifetime.

/// Returns a string literal, which lives for the whole program.
fn my_string() -> &'static str {
    // This string literal is stored directly in the binary,
    // and therefore has a `'static` lifetime.
    let s: &'static str = "I have a static lifetime.";
    s
}

/// Prints a `'static` string.
fn main() {
    println!("{}", my_string());
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
