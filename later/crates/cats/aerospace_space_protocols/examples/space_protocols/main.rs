//! Runs the space protocols example.
//!
//! TODO Reviewed ??-??-2026.

mod pus_telecommand;
mod pus_telemetry;
mod space_protocols;

/// Runs the space protocols example.
fn main() {
    space_protocols::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
