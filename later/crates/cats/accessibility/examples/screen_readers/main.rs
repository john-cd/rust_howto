//! Runs the `accesskit` accessibility example.
//!
//! TODO Reviewed ??-??-2026.

mod accesskit;

/// Runs the screen readers example.
fn main() {
    accesskit::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
