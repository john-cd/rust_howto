//! Runs the `rangemap` example.
//!
//! TODO Reviewed ??-??-2026.

/// A `RangeMap`, which maps ranges of keys to values.
mod rangemap;
/// Runs the example.
/// Runs the rangemap example.

fn main() {
    rangemap::run();
}

/// Verifies that the `rangemap` example.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rangemap_dispatcher_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
