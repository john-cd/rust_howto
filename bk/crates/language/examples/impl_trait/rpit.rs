#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Rust allows to write `impl Trait` as the return type of functions (often
//! called "RPIT"). This means that the function returns "some type that
//! implements the Trait". This is commonly used to return closures, iterators,
//! and other types that are complex or impossible to write explicitly.

/// This function returns a closure that takes an `i32` and returns an `i32`.
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

/// Calls a function that returns a closure with `impl Trait`.
fn main() {
    let f = returns_closure();
    println!("{}", f(1));
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rpit example through its entry point.
    #[test]
    fn test_rpit_example_entry_point() {
        main();
    }
}
