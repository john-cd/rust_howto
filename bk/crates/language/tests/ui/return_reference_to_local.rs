// TODO Reviewed ??-??-2026.

// ANCHOR: example
/// Intentionally returns a reference to a local allocation.
fn try_create<'a>() -> &'a String {
    let s = String::new();
    &s
}

/// Runs the example demonstrated by this module.
fn main() {
    let _ = try_create();
}
// ANCHOR_END: example
