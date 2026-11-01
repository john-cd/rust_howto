// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to use the `assert!` macro with a custom
//! failure message.
//!
//! A custom message in `assert!` explains why a check failed, in terms of the
//! problem instead of just two values. Include the inputs, so a failure in
//! continuous integration can be understood without rerunning it.

/// Asserts that `name` contains "Carol", with a custom failure message.
fn check_name(name: &str) {
    assert!(
        name.contains("Carol"),
        "Name did not contain 'Carol', value was `{name}`"
    );
}

/// Checks a name that does not contain "Carol", so that the assertion fails.
fn main() {
    println!("Checking if 'Carl' contains 'Carol'...");
    check_name("Carl");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    /// Runs the tests custom message example through its entry point.
    #[test]
    fn test_tests_custom_message_example_entry_point() {
        main();
    }
}
