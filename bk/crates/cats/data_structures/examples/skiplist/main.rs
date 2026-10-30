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

    /// Verifies that the skip-list entry point runs without panicking.
    #[test]
    fn skiplist_dispatcher_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
