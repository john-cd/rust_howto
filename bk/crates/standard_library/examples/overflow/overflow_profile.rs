// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Shows the default primitive integer overflow behavior for the active Cargo profile.

/// Adds one using ordinary primitive integer addition.
#[inline(never)]
fn add_one(value: u8) -> u8 {
    std::hint::black_box(value) + 1
}

/// Adds one to `u8::MAX`; default debug settings panic, while default release
/// settings wrap to zero.
fn main() {
    let value = u8::MAX;
    println!("{value} + 1 = {}", add_one(value));
}
// ANCHOR_END: example

/// Shows the default primitive integer overflow behavior for the active Cargo profile.
pub fn run() {
    main();
}

/// Verifies ordinary addition under the active Cargo profile's overflow setting.
#[cfg(test)]
mod tests {
    /// Checked arithmetic panics in the default debug profile.
    #[cfg(debug_assertions)]
    #[test]
    fn debug_profile_checks_overflow() {
        assert!(std::panic::catch_unwind(|| super::add_one(u8::MAX)).is_err());
    }

    /// The default release profile wraps primitive integer overflow.
    #[cfg(not(debug_assertions))]
    #[test]
    fn release_profile_wraps_overflow() {
        assert_eq!(super::add_one(u8::MAX), 0);
    }
}
