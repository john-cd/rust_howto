//! Runs the satellite tracking example.
//!
//! TODO Reviewed ??-??-2026.

mod aero;

/// Runs the aerospace example.
fn main() {
    aero::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
