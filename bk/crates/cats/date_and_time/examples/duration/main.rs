//! Runs the duration profiling example.
//!
//! TODO Reviewed ??-??-2026.

mod profile;

/// Runs the duration example.
fn main() {
    profile::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
