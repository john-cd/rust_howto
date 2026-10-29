//! Runs perception examples that need no physical sensors.
//!
//! TODO Reviewed ??-??-2026.

mod nearest_neighbor;

/// Looks up the nearest point in an in-memory point cloud.
fn main() -> anyhow::Result<()> {
    nearest_neighbor::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
