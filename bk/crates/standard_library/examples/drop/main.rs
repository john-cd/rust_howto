//! Runs the `Drop` trait example.
//!
//! TODO Reviewed ??-??-2026.

mod drop;

/// Runs the drop example.
fn main() {
    drop::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
