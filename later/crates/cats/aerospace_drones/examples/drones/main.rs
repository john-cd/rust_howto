//! Runs the drone example.
//!
//! TODO Reviewed ??-??-2026.

mod drones1;

/// Runs the drones example.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    drones1::run().map_err(std::convert::Into::into)
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
