//! Runs the `Pin` example.
//!
//! TODO Reviewed ??-??-2026.

mod pin;

/// Runs the pin example.
fn main() {
    pin::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
