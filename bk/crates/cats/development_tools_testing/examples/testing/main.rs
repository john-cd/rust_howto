//! Runs the `insta` snapshot testing example.
//!
//! TODO Reviewed ??-??-2026.

mod insta;

/// Runs the testing example.
fn main() {
    insta::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
