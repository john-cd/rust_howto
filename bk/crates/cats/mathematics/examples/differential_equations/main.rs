//! Runs a differential equations example.
//!
//! TODO Reviewed ??-??-2026.

mod exponential_decay;

/// Runs the exponential decay example.
fn main() {
    exponential_decay::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
