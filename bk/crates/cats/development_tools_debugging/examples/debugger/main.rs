// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A minimal program for stepping through with a debugger.

/// Computes and prints a value that can be inspected in a debugger.
/// A minimal program for stepping through with a debugger.
fn main() {
    let answer = 40 + 2;
    assert_eq!(answer, 42);
    println!("{answer}");
}
// ANCHOR_END: example

/// A minimal program for stepping through with a debugger.
fn run() {
    main();
}

#[cfg(test)]
mod tests {
    /// Runs the debugger example and checks its assertion.
    #[test]
    fn test_main_runs() {
        super::main();
    }
}
