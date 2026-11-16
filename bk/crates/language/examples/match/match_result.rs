//! Shows matching both variants of a result.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
/// Destructures a result and produces a message for either variant.
fn describe(res: Result<i32, String>) -> String {
    match res {
        Ok(value) => format!("Result: {value}"),
        Err(error) => format!("Error: {error}"),
    }
}
// ANCHOR_END: example

/// Each arm binds and formats its own payload.
#[test]
fn describes_success_and_error() {
    assert_eq!(describe(Ok(42)), "Result: 42");
    assert_eq!(describe(Err("unavailable".into())), "Error: unavailable");
}
