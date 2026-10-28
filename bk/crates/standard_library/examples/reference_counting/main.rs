//! Runs the reference counting example.
//!
//! TODO Reviewed ??-??-2026.

mod rc;

/// Runs the reference counting example.
fn main() {
    rc::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
