// ANCHOR: example

// Snapshot testing with the `insta` crate keeps track of how your data
// structures or output change over time.

// This is a sample function that formats user details,
// in e.g. `src/lib.rs`
pub fn format_user(name: &str, age: u32) -> String {
    format!("Name: {}, Age: {}", name, age)
}

// The following is a test that uses `insta`
// to snapshot the output, in e.g. `tests/snapshot.rs`.
// Run with `cargo test`

use insta::assert_snapshot;

#[test]
fn test_format_user() {
    let formatted = format_user("Alice", 30);
    // If the test fails, `insta` will prompt you to review and accept the new
    // snapshot. This allows you to easily manage changes to your output
    // over time.
    assert_snapshot!("format_user_snapshot", formatted);
}
// ANCHOR_END: example

// [P1](https://github.com/john-cd/rust_howto/issues/750)
