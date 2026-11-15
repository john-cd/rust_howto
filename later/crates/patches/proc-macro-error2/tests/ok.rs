// TODO Reviewed ??-??-2026.

use test_crate::*;

ok!(it_works);

/// Checks the it works invariants before ok is accepted by downstream operations.
#[test]
fn check_it_works() {
    it_works();
}
