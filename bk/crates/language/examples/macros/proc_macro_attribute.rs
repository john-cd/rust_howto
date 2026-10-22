#![allow(unused)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! An attribute procedural macro that logs function calls.

// Import our custom attribute macro.
use proc_macros::log_calls;

// Apply the custom attribute macro.
// This will log the function call.
/// Adds two numbers; calls are logged by the attribute macro.
#[log_calls]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Calls a function that is logged.
fn main() {
    let sum = add(5, 7);
    println!("Sum: {sum}");
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
