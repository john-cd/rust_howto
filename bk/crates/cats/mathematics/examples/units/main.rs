//! Runs the units of measurement example.
//!
//! TODO Reviewed ??-??-2026.

mod quantities_and_conversions;

/// Runs the quantity conversion example.
fn main() {
    quantities_and_conversions::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
