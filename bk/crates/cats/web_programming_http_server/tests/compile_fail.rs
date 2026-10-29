//! Checks the `compile_fail` snippet in the axum add-ons page with `trybuild`.

/// The handler with a body extractor before `State` must fail to compile with
/// the diagnostic that `#[debug_handler]` produces.
#[test]
fn debug_handler_rejects_a_misplaced_body_extractor() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/*.rs");
}
