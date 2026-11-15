//! Runs the `petgraph` example.
//!
//! TODO Reviewed ??-??-2026.

/// Exercises graph traversal and shortest-path algorithms with `petgraph`.
mod petgraph;

/// Runs the graph traversal example.
fn main() {
    petgraph::run();
}

/// Confirms that the graph example completes without panicking.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the graph dispatcher runs.
    #[test]
    fn graph_dispatcher_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
