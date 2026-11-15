//! Runs the `petgraph` example.
//!
//! TODO Reviewed ??-??-2026.

/// Builds or traverses graph graph data while preserving relationships needed by downstream
/// analysis. Workspace and rendering code use this module to obtain a consistent view of the graph.
mod petgraph;
/// Runs the example.
/// Runs the graph example.
fn main() {
    petgraph::run();
}

/// Verifies that the `petgraph` example.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the graph dispatcher runs.
    #[test]
    fn graph_dispatcher_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
