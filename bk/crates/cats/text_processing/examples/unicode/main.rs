//! Runs the Unicode grapheme example.
//!
//! TODO Reviewed ??-??-2026.

mod graphemes;

/// Runs the unicode example.
fn main() {
    graphemes::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
