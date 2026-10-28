//! Runs the MAVLink example.
//!
//! TODO Reviewed ??-??-2026.

mod aero_protocols;

/// Runs the aerospace protocols example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    aero_protocols::run()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
