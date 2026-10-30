//! Runs the `petgraph` example.
//!
//! TODO Reviewed ??-??-2026.

mod petgraph;

/// Runs the graph example.
fn main() {
    petgraph::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the graph entry point runs its example without panicking.
    #[test]
    fn graph_dispatcher_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
