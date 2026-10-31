// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates a simple panic.
//! When executed, it will cause the program to terminate abruptly.
//!
//! A panic is an unrecoverable error: the thread unwinds (running destructors)
//! and normally the program ends. Use `Result` for expected failures such as
//! bad input, and keep panics for bugs, for example a violated invariant.

/// This example demonstrates a simple panic.
/// When executed, it will cause the program to terminate abruptly.
fn main() {
    println!(
        "panic example: calling panic! causes immediate program termination"
    );
    panic!("Crash and burn");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    /// Runs the panic example through its entry point.
    #[test]
    fn test_panic_example_entry_point() {
        main();
    }
}
