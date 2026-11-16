//! Runs the artificial intelligence examples.
//!
//! TODO Reviewed ??-??-2026.

/// Robot path planning on an occupancy grid using the A* algorithm.
mod astar;
/// Controls a counter with a behavior tree built using `bonsai_bt`.
mod bonsai_bt;

/// Runs the artificial intelligence examples.
fn main() -> anyhow::Result<()> {
    astar::run()?;
    bonsai_bt::run()
}
