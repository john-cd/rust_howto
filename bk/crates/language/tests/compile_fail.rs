//! Checks that an incompatible mutable reborrow fails to compile.

/// Keeps the invalid mutation of a live shared reborrow from compiling.
#[test]
fn mutable_access_conflicts_with_live_shared_reborrow() {
    trybuild::TestCases::new()
        .compile_fail("tests/ui/incompatible_shared_reborrow.rs");
}
