// TODO Reviewed ??-??-2026.

/// Checks compiler output for expected procedural macro diagnostics.
#[test]
#[cfg(run_ui_tests)]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
