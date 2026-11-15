//! Runs perception examples that need no physical sensors.
//!
//! TODO Reviewed ??-??-2026.

/// Finds the closest measurement in a small three-dimensional point cloud.
mod nearest_neighbor;

/// Looks up the nearest point in an in-memory point cloud.
fn main() -> anyhow::Result<()> {
    nearest_neighbor::run()
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
