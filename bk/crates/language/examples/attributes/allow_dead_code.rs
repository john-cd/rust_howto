// TODO Reviewed ??-??-2026.

#![allow(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Silencing the warning about unused code with `#[allow(dead_code)]`.

// Disables the `dead_code` lint for the entire file.

/// This function is defined but never called.
///
/// The attribute above disables the `dead_code` lint,
/// which would otherwise warn about this unused function.
///
/// You could also add a `#[allow(dead_code)]` attribute here
/// to disable the lint for this function only.
fn unused_function() {}

/// Does not call `unused_function`.
fn main() {
    println!("Nobody is calling `unused_function`.");
}

// ANCHOR_END: example

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
}
