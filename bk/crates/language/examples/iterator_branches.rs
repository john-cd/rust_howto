//! Returns two iterator implementations through a generated enum.

// ANCHOR: example
use auto_enums::auto_enum;

/// Returns one default retry delay or the full backoff schedule, in milliseconds.
#[auto_enum(Iterator)]
fn retry_delays(quick_retry: bool) -> impl Iterator<Item = u64> {
    if quick_retry {
        std::iter::once(100)
    } else {
        [100, 200, 400].into_iter()
    }
}

/// Checks the schedules produced by the two iterator branches.
fn main() {
    assert!(retry_delays(true).eq([100]));
    assert!(retry_delays(false).eq([100, 200, 400]));
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Checks both returned iterators produce their expected values in order.
    #[test]
    fn iterates_both_branches() {
        super::main();
    }
}
