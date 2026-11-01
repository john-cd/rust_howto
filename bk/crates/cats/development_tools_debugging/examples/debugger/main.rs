//! A minimal program for stepping through with a debugger.

// ANCHOR: example
/// Computes and prints a value that can be inspected in a debugger.
fn main() {
    let answer = 40 + 2;
    assert_eq!(answer, 42);
    println!("{answer}");
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Runs the debugger example and checks its assertion.
    #[test]
    fn test_main_runs() {
        super::main();
    }
}
