//! Runs the linked list example.
//!
//! TODO Reviewed ??-??-2026.

/// A `LinkedList`, a doubly linked list.
mod linkedlist;
/// Runs the example.
/// Runs the linkedlist example.

fn main() {
    linkedlist::run();
}

/// Verifies that the linked list example.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linkedlist_dispatcher_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
