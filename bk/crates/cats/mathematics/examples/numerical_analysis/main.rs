//! Runs the numerical analysis example.
//!
//! TODO Reviewed ??-??-2026.

mod root_finding;

/// Runs the root-finding example.
fn main() {
    root_finding::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
