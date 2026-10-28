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

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
