#![allow(unused)]
// TODO Reviewed ??-??-2026.

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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the proc macro attribute example through its entry point.
    #[test]
    fn test_proc_macro_attribute_example_entry_point() {
        main();
    }

    /// The attribute leaves the function's behavior intact.
    #[test]
    fn test_logged_function_still_adds() {
        assert_eq!(add(5, 7), 12);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
}
