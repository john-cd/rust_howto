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

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
