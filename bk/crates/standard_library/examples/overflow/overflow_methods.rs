// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! Demonstrates explicit primitive integer overflow operations.

/// Compares wrapping, checked, overflowing, and saturating addition.
/// Demonstrates explicit primitive integer overflow operations.
fn main() {
    let x = 255_u8;

    assert_eq!(x.wrapping_add(1), 0);
    assert_eq!(x.checked_add(1), None);
    assert_eq!(x.overflowing_add(1), (0, true));
    assert_eq!(x.saturating_add(1), 255);
}
// ANCHOR_END: example

/// Runs this example for its parent module.
/// Demonstrates explicit primitive integer overflow operations.
pub(super) fn run() {
    main();
}

/// Verifies the four explicit overflow behaviors at the `u8` boundary.
#[cfg(test)]
mod tests {

    /// Verifies the four explicit overflow behaviors at the `u8` boundary.
    #[test]
    fn test_explicit_overflow_methods() {
        super::main();
    }
}
