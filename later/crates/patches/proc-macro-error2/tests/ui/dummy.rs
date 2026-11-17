// TODO Reviewed ??-??-2026.

use test_crate::*;

/// Defines the outcomes available to the ui workflow, including a, b.
enum NeedDefault {
    /// Represents the a case of need default.
    A,
    /// Represents the b case of need default.
    B,
}

dummy!(need_default);

/// Runs the example demonstrated by this module.
fn main() {
    let _ = NeedDefault::default();
}
