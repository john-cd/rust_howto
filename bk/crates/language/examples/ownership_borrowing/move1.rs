// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates move semantics in Rust.
//!
//! Note that `String` (and all non-`Copy` types) have "move" semantics.

/// Moves a `String` from one variable to another.
fn main() {
    // The variable `s1` owns the string "hello".
    let s1 = String::from("hello");

    // The value in `s1` is MOVED into `s2`.
    // `s2` now owns the string "hello" and Rust invalidates `s1`.
    // This is NOT a shallow copy or a deep copy.
    let s2 = s1;

    println!("{s2}, world!");
    // ERROR: println!("{s1}, world!"); // `s1` is invalid.
}

// `s2` gets out of scope here, therefore the String it owns is dropped
// (deallocated). `s1` invalidation earlier prevents a "double free" error,
// where two variables might try to deallocate the same memory when they go out
// of scope.
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the move1 example through its entry point.
    #[test]
    fn test_move1_example_entry_point() {
        main();
    }

    // Output-only example: `main` only prints values; there is nothing else to
    // check.
}
