// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A minimal program for stepping through with a debugger.

/// Computes and prints a value that can be inspected in a debugger.
/// A minimal program for stepping through with a debugger.
fn main() {
    example();
}

/// Runs the debugger example's calculation and assertion.
fn example() {
    let answer = 40 + 2;
    assert_eq!(answer, 42);
    println!("{answer}");
}
// ANCHOR_END: example

/// A minimal program for stepping through with a debugger.
pub fn run() {
    let entry_point = main;
    entry_point();
}

/// Provides private main tests behavior for the debugger walkthrough component.
#[cfg(test)]
mod private_main_tests {
    use super::main;

    /// Checks that private main succeeds.
    #[test]
    fn private_main_succeeds() {
        let entry_point = main;
        entry_point();
    }
}
