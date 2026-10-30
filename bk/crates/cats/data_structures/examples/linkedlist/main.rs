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

    /// Verifies that the linked-list entry point runs without panicking.
    #[test]
    fn linkedlist_dispatcher_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
