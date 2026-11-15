//! Runs the lexer example.
//!
//! TODO Reviewed ??-??-2026.

/// Tokenizes a small arithmetic expression with logos.
mod tokenization;
/// Runs the example.
/// Runs the lexer example.

fn main() {
    tokenization::run();
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
