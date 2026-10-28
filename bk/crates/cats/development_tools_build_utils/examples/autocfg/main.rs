//! Autocfg examples.
//!
//! TODO Reviewed ??-??-2026.

mod autocfg;

/// Runs the autocfg example.
fn main() {
    autocfg::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
