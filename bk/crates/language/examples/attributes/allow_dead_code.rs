// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Silencing the warning about unused code with `#[allow(dead_code)]`.
//!
//! The Rust compiler generates warnings for code that is defined but never
//! referenced, helping prevent dead code accumulation. The `dead_code` lint
//! can be silenced using `#[allow(dead_code)]` when developing or prototyping.

/// This function is defined but never called.
///
/// In production code, unused functions should typically be removed to keep
/// the codebase clean and maintainable. In early development or prototyping,
/// the attribute below suppresses the `dead_code` compiler warning.
#[allow(dead_code)]
fn unused_function() {}

/// Does not call `unused_function`.
fn main() {
    println!("Nobody is calling `unused_function`.");
    // Expected output: Nobody is calling `unused_function`.
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the allow dead code example through its entry point.
    #[test]
    fn test_allow_dead_code_example_entry_point() {
        main();
    }

    // Output-only example: the unused function is deliberately never called.
}
