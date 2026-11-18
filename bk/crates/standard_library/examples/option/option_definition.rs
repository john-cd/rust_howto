//! Models optional values and compares explicit and implicit early return.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: definition
/// Simplified optional-value model: either no value or one owned payload.
enum Option<T> {
    /// No value is present.
    None,
    /// One value is present.
    Some(
        /// Owned value represented by the optional container.
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
