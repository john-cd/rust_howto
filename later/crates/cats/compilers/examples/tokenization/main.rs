//! Runs the lexer example.

mod tokenization;

/// Runs the lexer example.
fn main() {
    tokenization::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
