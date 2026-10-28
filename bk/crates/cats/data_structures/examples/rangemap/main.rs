//! Runs the `rangemap` example.
//!
//! TODO Reviewed ??-??-2026.

mod rangemap;

/// Runs the rangemap example.
fn main() {
    rangemap::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
