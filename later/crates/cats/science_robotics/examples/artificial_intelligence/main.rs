//! Runs the artificial intelligence examples.
//!
//! TODO Reviewed ??-??-2026.

/// Robot path planning on an occupancy grid using the A* algorithm.
mod astar;
/// # Behavior Tree Example This example demonstrates the use of a Behavior Tree (BT) to control a
/// simple counter.
mod bonsai_bt;

/// Runs the artificial intelligence examples.
fn main() -> anyhow::Result<()> {
    astar::run()?;
    bonsai_bt::run()
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
