//! Checks that an incompatible mutable reborrow fails to compile.
//!
//! TODO Reviewed ??-??-2026.

/// Keeps the invalid mutation of a live shared reborrow from compiling.
#[test]
fn mutable_access_conflicts_with_live_shared_reborrow() {
    trybuild::TestCases::new().compile_fail("tests/ui/incompatible_shared_reborrow.rs");
}

/// Checks the derive macro's message and span for unsupported input.
#[test]
fn debug_print_rejects_unions() {
    trybuild::TestCases::new().compile_fail("tests/ui/debug_print_union.rs");
}

/// Rejects the invalid references, non-Send future, and unsupported macro input.
#[test]
fn chapter_diagnostics() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/return_reference_to_local.rs");
    cases.compile_fail("tests/ui/shorter_input_lifetime.rs");
    cases.compile_fail("tests/ui/future_holds_non_send_value.rs");
    cases.compile_fail("tests/ui/non_identifier_macro.rs");
}
