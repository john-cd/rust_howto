//! Runs the append-only collection examples.

mod frozen_map;
mod frozen_vec;

/// Runs both `elsa` collection examples.
fn main() {
    frozen_map::run();
    frozen_vec::run();
}

#[cfg(test)]
mod tests {
    /// Runs both collection examples and their assertions.
    #[test]
    fn test_examples_run() {
        super::main();
    }
}
