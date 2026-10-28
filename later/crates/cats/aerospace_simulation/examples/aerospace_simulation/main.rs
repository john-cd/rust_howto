//! Runs the rocket simulation example.
//!
//! TODO Reviewed ??-??-2026.

mod aero_simulation;

/// Runs the aerospace simulation example.
fn main() {
    aero_simulation::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
