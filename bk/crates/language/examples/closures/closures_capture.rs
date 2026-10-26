#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Closures that capture their environment.

/// Defines a closure that captures a variable and calls it.
fn main() {
    let outer_variable = 10;

    // Define a simple closure that takes a parameter and captures
    // `outer_variable`:
    let my_closure = |x| {
        println!("Parameter x = {x}");
        println!("Outer variable = {outer_variable}"); // Captured variable.
        x + outer_variable // Optional return value.
    }; // The body can be an expression or block between `{` and `}`.

    // Call the closure:
    let result = my_closure(5);
    println!("Result from closure: {result}"); // Output: 15.
}

// ANCHOR_END: example

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
