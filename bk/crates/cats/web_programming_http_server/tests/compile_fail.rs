//! Checks the `compile_fail` snippet in the axum add-ons page with `trybuild`.
//!
//! TODO Reviewed ??-??-2026.

/// Rejects an Axum handler with a misplaced body extractor.
#[test]
fn debug_handler_rejects_a_misplaced_body_extractor() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/*.rs");
}
