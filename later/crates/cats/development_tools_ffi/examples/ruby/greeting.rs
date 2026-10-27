//! Ruby-independent greeting logic shared with the native example.

/// Returns the text exposed by the Ruby example.
pub fn message() -> String {
    String::from("Hello from Rust again!")
}

#[cfg(test)]
mod tests {
    /// Checks the return value without initializing or linking a Ruby VM.
    #[test]
    fn message_matches_the_ruby_example() {
        assert_eq!(super::message(), "Hello from Rust again!");
    }
}
