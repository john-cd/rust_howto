//! Placeholder for examples of software written in Rust.
//!
//! Reviewed 10-26-2026.

// ANCHOR: example
/// Prints a notice that this page has no executable project integration.
fn main() {
    println!("No written_in_rust examples are implemented yet.");
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    // output-only: the example is a placeholder that only prints a notice.
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
