//! Runs perception examples that need no physical sensors.
//!
//! TODO Reviewed ??-??-2026.

mod nearest_neighbor;

/// Looks up the nearest point in an in-memory point cloud.
fn main() -> anyhow::Result<()> {
    nearest_neighbor::run()
}
