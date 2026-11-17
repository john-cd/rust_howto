//! Models the two variants of the standard result type.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
enum Result<T, E> {
    /// Represents the ok case of result.
    Ok(
        /// Value produced when the operation succeeds.
        T,
    ),
    /// Represents the err case of result.
    Err(
        /// Error produced when the operation fails.
        E,
    ),
}
// ANCHOR_END: example

/// Success and failure carry values of different types.
#[test]
fn variants_carry_distinct_payloads() {
    let success: Result<u8, &str> = Result::Ok(7);
    let failure: Result<u8, &str> = Result::Err("invalid");
    assert!(matches!(success, Result::Ok(7)));
    assert!(matches!(failure, Result::Err("invalid")));
}
