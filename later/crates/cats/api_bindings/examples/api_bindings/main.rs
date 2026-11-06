// TODO Reviewed ??-??-2026.

//! Placeholder for API bindings examples.

// Deferred (#1748): no API bindings example yet.

// ANCHOR: example
/// The notice printed while this category has no implemented example.
const PLACEHOLDER_NOTICE: &str = "No API bindings example is implemented yet.";

/// Prints a notice that this category still needs a bindings example.
fn main() {
    println!("{PLACEHOLDER_NOTICE}");
}
// ANCHOR_END: example

/// Tests the placeholder entry point and its message.
#[cfg(test)]
mod tests {
    /// Verifies the placeholder message and runs the example entry point.
    #[test]
    fn test_main_entry_point() {
        assert_eq!(
            super::PLACEHOLDER_NOTICE,
            "No API bindings example is implemented yet."
        );
        super::main();
    }
}
