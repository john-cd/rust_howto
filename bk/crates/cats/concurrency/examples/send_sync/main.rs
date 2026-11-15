//! Runs the `Send` and `Sync` example.
//!
//! TODO Reviewed ??-??-2026.

/// Simplified implementation of the `Arc` smart pointer.
mod send_sync_impl;
/// Runs the example.
/// Runs the send sync example.
fn main() {
    send_sync_impl::run();
}

/// panic fails the test.
#[cfg(test)]
mod tests {
    use super::*;

    /// panic fails the test.
    #[test]
    fn test_main_runs() {
        main();
    }
}
