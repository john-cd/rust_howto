//! Runs examples of explicit integer overflow behavior.

mod overflow_methods;
mod wrapping_type;

/// Runs the primitive methods and `Wrapping` examples.
fn main() {
    overflow_methods::run();
    wrapping_type::run();
}

#[cfg(test)]
mod tests {
    /// Runs both overflow examples and their assertions.
    #[test]
    fn test_examples_run() {
        super::main();
    }
}
