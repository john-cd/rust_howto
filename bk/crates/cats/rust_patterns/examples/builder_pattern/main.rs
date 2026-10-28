//! Runs the builder pattern example.
//!
//! TODO Reviewed ??-??-2026.

mod builder_pattern;

/// Runs the builder pattern example.
fn main() {
    builder_pattern::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
