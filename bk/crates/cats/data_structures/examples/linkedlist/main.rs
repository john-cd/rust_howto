//! Runs the linked list example.
//!
//! TODO Reviewed ??-??-2026.

mod linkedlist;

/// Runs the linkedlist example.
fn main() {
    linkedlist::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
