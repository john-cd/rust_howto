//! Runs the skip list example.
//!
//! TODO Reviewed ??-??-2026.

mod skiplist;

/// Runs the skiplist example.
fn main() {
    skiplist::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
