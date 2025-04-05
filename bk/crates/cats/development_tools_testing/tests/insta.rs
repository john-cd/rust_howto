// ANCHOR: example
//! Snapshot testing with the `insta` crate keeps track of how your data
//! structures or output change over time.
use insta::assert_snapshot;

/// This is a sample function that formats user details.
pub fn format_user(name: &str, age: u32) -> String {
    format!("Name: {}, Age: {}", name, age)
}

/// The following is a test that uses `insta` to snapshot the output.
/// Run with `cargo test`.
/// If the test fails, `insta` will prompt you to review and accept the new
/// snapshot. This allows you to easily manage changes to your output
/// over time.
#[test]
fn test_format_user() {
    let formatted = format_user("Alice", 30);
    assert_snapshot!("format_user_snapshot", formatted);
}
// ANCHOR_END: example
