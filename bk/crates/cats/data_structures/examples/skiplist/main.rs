//! Runs the skip list example.
//!
//! TODO Reviewed ??-??-2026.

/// A `SkipMap`, an ordered map built on a skip list.
mod skiplist;
/// Runs the example.
/// Runs the skiplist example.

fn main() {
    skiplist::run();
}

/// Verifies that the skip list example.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skiplist_dispatcher_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
