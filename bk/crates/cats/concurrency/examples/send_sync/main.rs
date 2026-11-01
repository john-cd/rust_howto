//! Runs the `Send` and `Sync` example.
//!
//! TODO Reviewed ??-??-2026.

mod send_sync_impl;

/// Runs the send sync example.
fn main() {
    send_sync_impl::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example; each module's `run()` asserts its own results, so a
    /// panic fails the test.
    #[test]
    fn test_main_runs() {
        main();
    }
}
