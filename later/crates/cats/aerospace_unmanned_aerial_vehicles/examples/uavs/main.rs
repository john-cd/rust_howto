//! Runs the UAV example.
//!
//! TODO Reviewed ??-??-2026.

mod uavs;

/// Runs the uavs example.
fn main() -> anyhow::Result<()> {
    uavs::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    // output-only: the example talks to a real drone.
    #[test]
    #[ignore = "requires real drone"]
    fn test_main_entry_point() -> anyhow::Result<()> {
        main()
    }
}
