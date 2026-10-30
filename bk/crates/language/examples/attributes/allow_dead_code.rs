// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Silencing the warning about unused code with `#[allow(dead_code)]`.

/// This function is defined but never called.
///
/// The attribute below disables the `dead_code` lint for this function.
#[allow(dead_code)]
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

    // Output-only example: the unused function is deliberately never called.
}
