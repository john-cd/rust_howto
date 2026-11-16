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
