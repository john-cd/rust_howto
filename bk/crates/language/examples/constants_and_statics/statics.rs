#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! `static` Example.

// Statics are in SCREAMING_SNAKE_CASE, must declare their type, must be
// initialized:
/// The name of the application.
static APPLICATION_NAME: &str = "My Awesome App";

/// The version of the application.
static VERSION: u32 = 1;

/// Prints statics and shows that they do not `Drop`.
fn main() {
    println!("Welcome to {APPLICATION_NAME} v{VERSION}");

    // Statics do not `Drop`.
    /// Static value whose `Drop` implementation is not run at program exit.
    static POD: PrintOnDrop = PrintOnDrop("This message does not appear.");
}

// A struct that implements a `Drop` destructor.
/// A type that prints a message when it is dropped.
struct PrintOnDrop(&'static str);

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the statics example through its entry point.
    #[test]
    fn test_statics_example_entry_point() {
        main();
    }
}
