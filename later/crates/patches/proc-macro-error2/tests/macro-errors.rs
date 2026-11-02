// TODO Reviewed ??-??-2026

#[test]
#[cfg(run_ui_tests)]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
