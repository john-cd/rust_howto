//! Runs the `FromStr` parsing example.
//!
//! TODO Reviewed ??-??-2026.

mod from_str;

/// Runs the string parsing example.
fn main() {
    from_str::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
