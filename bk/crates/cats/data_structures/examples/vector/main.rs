//! Runs the vector example.
//!
//! TODO Reviewed ??-??-2026.

mod vectors;

/// Entry point for the vector example.
fn main() {
    vectors::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
