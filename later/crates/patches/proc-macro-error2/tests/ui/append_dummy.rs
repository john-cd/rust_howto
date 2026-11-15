// TODO Reviewed ??-??-2026.

use test_crate::*;

/// Defines the outcomes available to the ui workflow, including a, b.
enum NeedDefault {
    A,
    B,
}

append_dummy!(need_default);

fn main() {
    let _ = NeedDefault::default();
}
