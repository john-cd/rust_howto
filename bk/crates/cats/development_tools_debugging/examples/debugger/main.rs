// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A minimal program for stepping through with a debugger.

/// Computes and prints a value that can be inspected in a debugger.
/// A minimal program for stepping through with a debugger.
fn main() {
    run();
}

/// Runs the debugger example's calculation and assertion.
fn example() {
    let answer = 40 + 2;
    assert_eq!(answer, 42);
    println!("{answer}");
}
// ANCHOR_END: example

/// A minimal program for stepping through with a debugger.
fn run() {
    example();
}
