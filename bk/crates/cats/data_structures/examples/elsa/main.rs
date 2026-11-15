//! Runs the append-only collection examples.
//!
//! TODO Reviewed ??-??-2026.

/// Demonstrates stable references in an `elsa::FrozenMap`.
mod frozen_map;
/// Demonstrates stable references in an `elsa::FrozenVec`.
mod frozen_vec;
/// Runs the example.
/// Runs both `elsa` collection examples.

fn main() {
    frozen_map::run();
    frozen_vec::run();
}

/// Runs both collection examples and their assertions.
#[cfg(test)]
mod tests {
    /// Runs both collection examples and their assertions.
    #[test]
    fn test_examples_run() {
        super::main();
    }
}
