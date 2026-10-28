//! Runs the physics engines example.
//!
//! TODO Reviewed ??-??-2026.

mod rapier2d;

/// Runs the Rapier 2D example on every supported desktop target.
fn main() {
    rapier2d::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
