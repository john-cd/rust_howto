//! Runs the UAV example.
//!
//! TODO Reviewed ??-??-2026.

mod uavs;

/// Runs the uavs example.
fn main() {
    uavs::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
