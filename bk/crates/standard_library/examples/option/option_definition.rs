//! Models optional values and compares explicit and implicit early return.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: definition
enum Option<T> {
    /// Represents the none case of option.
    None,
    /// Represents the some case of option.
    Some(
        /// Successful result value returned to the caller.
        T,
    ),
}
// ANCHOR_END: definition

/// An optional value either carries its payload or represents its absence.
#[test]
fn presence_is_explicit() {
    let present = Option::Some(42);
    let absent = Option::<u8>::None;
    assert!(matches!(present, Option::Some(42)));
    assert!(matches!(absent, Option::None));
}
