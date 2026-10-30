//! Runs the `rangemap` example.
//!
//! TODO Reviewed ??-??-2026.

mod rangemap;

/// Runs the rangemap example.
fn main() {
    rangemap::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the range-map entry point runs without panicking.
    #[test]
    fn rangemap_dispatcher_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
