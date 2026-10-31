//! Snapshot testing with `insta`.
//!
//! A snapshot test compares the output with a reference file stored in the
//! repository. When the output changes on purpose, `cargo insta review` shows
//! the difference and lets you accept it. It is convenient for large outputs
//! such as generated text or JSON.

pub fn run() {
    main();
}

// TODO Reviewed ??-??-2026.

// ANCHOR: example
// Snapshot testing with the `insta` crate keeps track of how your data
// structures or output change over time.

/// This is a sample function that formats user details.
pub fn format_user(name: &str, age: u32) -> String {
    format!("Name: {name}, Age: {age}")
}

/// Prints a formatted user.
fn main() {
    println!("{}", format_user("Alice", 30));
}

/// The following is a test that uses `insta` to snapshot the output.
/// Run with `cargo test`.
/// If the test fails, `insta` will prompt you to review and accept the new
/// snapshot. This allows easily management of changes to your output
/// over time.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the formatted user against a stored snapshot.
    #[test]
    fn test_format_user() {
        use insta::assert_snapshot;

        let formatted = format_user("Alice", 30);
        assert_snapshot!(formatted, @"Name: Alice, Age: 30");
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
// ANCHOR_END: example
